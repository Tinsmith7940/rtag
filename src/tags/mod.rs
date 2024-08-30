use anyhow::Result;
use core::fmt::Debug;
use id3::*;
use std::ffi::OsStr;
use std::path::Path;
use mp4ameta::AudioInfo;
fn get_extension(path: &str) -> FileExtension {
    let ext = Path::new(path)
        .extension()
        .and_then(OsStr::to_str)
        .unwrap_or("");
    match ext {
        "mp3" => FileExtension::Mp3,
        "m4a" => FileExtension::M4a,
        _default => panic!("{ext} is not a supported file extension! Cannot determine file type"),
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
pub fn get_audiofile(path: String, clean_tag: bool) -> Box<dyn TagUtils> {
    let audio_type = get_extension(path.as_str());
    match audio_type {
        FileExtension::Mp3 => {
            log::info!("parsing mp3 file");
            let tag;
            if clean_tag {
                tag = Id3Tag::new(path);
            } else {
                tag = Id3Tag::create_tag_from_path(path);
            }
            Box::new(tag)
        }
        FileExtension::M4a => {
            log::info!("parsing m4a file");
            let tag;
            if clean_tag {
                tag = M4aTag::new(path);
            } else {
                tag = M4aTag::create_tag_from_path(path);
            }
            Box::new(tag)
        }
    }
}

/// Trait defining all basic read/write tag operations
pub trait TagUtils {
    fn artist(&self) -> Option<String>;

    fn set_artist(&mut self, artist: String);

    fn year(&self) -> Option<String>;

    fn set_year(&mut self, year: String);

    fn title(&self) -> Option<String>;

    fn set_title(&mut self, title: String);
    fn write_to_file(&self) -> Result<()>;
    fn clear(&mut self) -> Result<()>;
}

impl Debug for dyn TagUtils {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("TagUtils").finish()
    }
}

/// Trait defining entrypoint for all Tag initialization
pub trait Tag {
    fn new(file: String) -> Self;
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
    /// Create a new, empty, tag
    fn new(file: String) -> Self {
        let audio_info = AudioInfo::default();
        let tag = mp4ameta::Tag::new("".to_string(), audio_info, vec![]);

        M4aTag { tag, path: file }
    }
    /// Create a new tag, with existing atoms populated from target file
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

    fn set_year(&mut self, year: String) {
        let _ = &self.tag.set_year(year);
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

    fn clear(&mut self) -> Result<()> {
        let _ = self.tag.clear();
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

    fn new(file: String) -> Self {
        Id3Tag { 
            tag: id3::Tag::new(),
            path: file
        }
    }

    fn create_tag_from_path(file: String) -> Self {
        match id3::Tag::read_from_path(file.clone()) {
            Ok(tag) => Id3Tag { tag, path: file },
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

    fn set_year(&mut self, year: String) {
        let _ = &self.tag.set_year(year.parse::<i32>().unwrap());
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

    fn clear(&mut self) -> Result<()> {
        panic!("--clear is not implemented for id3 tag types (mp3, wav, etc...)");
    }
}

#[cfg(test)]
mod test {
    use crate::tags::{get_extension, FileExtension};

    #[test]
    fn match_file_extension() {
        let mp3 = "foo.mp3";
        let m4a = "bar.m4a";

        assert_eq!(get_extension(mp3), FileExtension::Mp3);
        assert_eq!(get_extension(m4a), FileExtension::M4a);
    }

    #[test]
    #[should_panic(expected = "foo is not a supported file extension! Cannot determine file type")]
    fn no_match_panics() {
        let mp3 = "bar.foo";
        get_extension(mp3);
    }
}
