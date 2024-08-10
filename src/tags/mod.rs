use core::fmt::Debug;
use id3::*;
use anyhow::Result;

fn get_extension(path: &str) -> FileExtension {
    let file = path.to_string();
    match file.split('.').last().unwrap().to_lowercase().as_str() {
        "mp3" => FileExtension::Mp3,
        "m4a" => FileExtension::M4a,
        _default => panic!("No valid file extension found! Cannot determine file type"),
    }
}

/// Enum of valid file extensions, including:
/// - Mp3
/// - M4a
#[derive(Debug, PartialEq, Eq)]
pub enum FileExtension {
    Mp3,
    M4a,
}

/// Parse the target file and build the correct
/// AudioFile enum value based on filetype
pub fn get_audiofile(path: String) -> Box<dyn TagUtils> {
    let audio_type = get_extension(path.as_str());
    match audio_type {
        FileExtension::Mp3 => {
            log::info!("parsing mp3 file");
            Box::new(Id3Tag::create_tag_from_path(path))
        }
        FileExtension::M4a => {
            log::info!("parsing m4a file");
            Box::new(M4aTag::create_tag_from_path(path))
        }
    }
}

/// Trait defining all basic read/write tag operations
pub trait TagUtils {
    fn artist(&self) -> Option<String>;

    fn set_artist(&mut self, artist: String);

    fn year(&self) -> Option<String>;

    fn set_year(&mut self, year: i32);

    fn title(&self) -> Option<String>;

    fn set_title(&mut self, title: String);
    fn write_to_file(&self) -> Result<()>;

}

impl Debug for dyn TagUtils {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("TagUtils").finish()
    }
}

/// Trait defining entrypoint for all Tag initialization
pub trait Tag {
    fn create_tag_from_path(file: String) -> Self;
}

/// Struct encapsulating m4ameta::Tag interface
#[derive(Debug, Clone)]
pub struct M4aTag {
    /// mp4a Tag struct
    tag: mp4ameta::Tag,
    /// file path
    path: String,
}

impl Tag for M4aTag {
    fn create_tag_from_path(file: String) -> Self {
        match mp4ameta::Tag::read_from_path(file.clone()) {
            Ok(tag) => M4aTag { tag, path: file },
            Err(e) => {
                panic!("Unable to parse provided file path. Error: {e}")
            }
        }
    }
}

impl TagUtils for M4aTag {
    fn artist(&self) -> Option<String> {
        self.tag.artist().map(|artist| artist.to_string())
    }

    fn set_artist(&mut self, artist: String) {
        let _ = &self.tag.set_artist(artist);
    }

    fn year(&self) -> Option<String> {
        self.tag.year().map(|year| year.to_string())
    }

    fn set_year(&mut self, year: i32) {
        let _ = &self.tag.set_year(year.to_string());
    }

    fn title(&self) -> Option<String> {
        self.tag.title().map(|title| title.to_string())
    }

    fn set_title(&mut self, title: String) {
        let _ = &self.tag.set_title(title);
    }

    fn write_to_file(&self) -> Result<()> {
        let _ = &self.tag.write_to_path(&self.path)?;
        Ok(())
    }
}

/// Struct encapsulating id3::Tag interface
#[derive(Debug, Clone)]
pub struct Id3Tag {
    /// Tag struct for interacting with id3 tags
    tag: id3::Tag,
    /// file path
    path: String,
}

impl Tag for Id3Tag {
    fn create_tag_from_path(file: String) -> Self {
        match id3::Tag::read_from_path(file.clone()) {
            Ok(tag) => Id3Tag { tag, path: file},
            Err(e) => {
                panic!("Unable to parse provided file path. Error: {e}")
            }
        }
    }
}
impl TagUtils for Id3Tag {
    fn artist(&self) -> Option<String> {
        self.tag.artist().map(|artist| artist.to_string())
    }

    fn set_artist(&mut self, artist: String) {
        let _ = &self.tag.set_artist(artist);
    }

    fn year(&self) -> Option<String> {
        self.tag.year().map(|year| year.to_string())
    }

    fn set_year(&mut self, year: i32) {
        let _ = &self.tag.set_year(year);
    }

    fn title(&self) -> Option<String> {
        self.tag.title().map(|title| title.to_string())
    }

    fn set_title(&mut self, title: String) {
        let _ = &self.tag.set_title(title);
    }

    fn write_to_file(&self) -> Result<()> {
        let _ = &self.tag.write_to_path(&self.path, Version::Id3v24)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::tags::{get_extension, FileExtension};

    #[test]
    fn match_file_extension() {
        let mp3 = "mp3";
        let m4a = "m4a";

        assert_eq!(get_extension(mp3), FileExtension::Mp3);
        assert_eq!(get_extension(m4a), FileExtension::M4a);
    }

    #[test]
    fn match_file_extension_case_insensitive() {
        let mp3 = "MP3";
        let m4a = "m4A";
        assert_eq!(get_extension(mp3), FileExtension::Mp3);
        assert_eq!(get_extension(m4a), FileExtension::M4a);
    }

    #[test]
    #[should_panic(expected = "No valid file extension found! Cannot determine file type")]
    fn no_match_panics() {
        let mp3 = "foo";
        get_extension(mp3);
    }
}
