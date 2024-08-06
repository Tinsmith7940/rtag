use clap::Parser;
use id3::*;
use simple_logger::SimpleLogger;
use core::fmt::Debug;
fn main() {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();

    let args = Args::parse();

    let audiotagbox = get_audiotag(args.file.clone());

    let audiotag = audiotagbox.clone();

    let clone_at = audiotag.get_audiofile().unwrap();
    log::info!("Audiotag loaded: {:?}",audiotag);
    println!("Artist: {:?}",clone_at.artist());
    println!("Artist: {:?}",clone_at.year());
}

/// A simple program to read and write audio file tags
#[derive(Parser, Debug)]
#[command(version, about, long_about = "None")]
pub struct Args {
    /// target audio file
    #[arg(short, long)]
    file: String,
}

pub fn get_extension(path: &str) -> FileExtension {
    let file = path.to_string();
    match file.split('.').last().unwrap().to_lowercase().as_str() {
        "mp3" => FileExtension::Mp3,
        "m4a" => FileExtension::M4a,
        _default => panic!("No valid file extension found! Cannot determine file type"),
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum FileExtension {
    Mp3,
    M4a
}
#[derive(Debug, Clone)]
pub enum AudioFile {
    Mp3(Mp3Tag),
    M4a(M4aTag)
}

impl AudioFile {

    fn get_audiofile(&self) -> Option<Box<&dyn TagUtils>> {
       match self {
            Self::Mp3(s) => Some(Box::new(s)),
            Self::M4a(s) => Some(Box::new(s)),
        }
    }
    fn get_mp3(self) -> Option<Mp3Tag> {
        match self {
            Self::Mp3(s) => Some(s),
            _ => None
        }
    }
    fn get_m4a(self) -> Option<M4aTag> {
        match self {
            Self::M4a(s) => Some(s),
            _ => None
        }
    }
}
pub fn get_audiotag(path: String) -> AudioFile {
    let audio_type = get_extension(path.as_str());
     match audio_type {
        FileExtension::Mp3 => {
            log::info!("parsing mp3 file");
            AudioFile::Mp3(Mp3Tag::create_tag_from_path(path))
        }
        FileExtension::M4a => {
            log::info!("parsing m4a file");
            AudioFile::M4a(M4aTag::create_tag_from_path(path))
       }
    }
}

pub trait TagUtils {

    fn artist(&self) -> Option<String>;

    fn year(&self) -> Option<String>;
}

impl Debug for dyn TagUtils {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("TagUtils").finish()
    }
}

pub trait Tag {
    fn create_tag_from_path(file: String) -> Self;
}

#[derive(Debug, Clone)]
pub struct M4aTag {
    tag: mp4ameta::Tag
}
impl Tag for M4aTag {
    fn create_tag_from_path(file: String) -> Self {
        match mp4ameta::Tag::read_from_path(file) {
            Ok(tag) => {
                M4aTag {
                    tag
                }
            },
            Err(e) => {
                panic!("Unable to parse provided file path. Error: {e}")
            }
        }
    }
}

impl TagUtils for M4aTag {
    fn artist(&self) -> Option<String> {
        match self.tag.artist() {
            Some(artist) => Some(artist.to_string()),
            _ => None
        }
    }

    fn year(&self) -> Option<String> {
        match self.tag.year() {
            Some(year) => Some(year.to_string()),
            _ => None,
        }
    }
}
#[derive(Debug, Clone)]
pub struct Mp3Tag {
    tag: id3::Tag
}

impl Tag for Mp3Tag {
    fn create_tag_from_path(file: String) -> Self {
        match id3::Tag::read_from_path(file) {
            Ok(tag) => {
                Mp3Tag {
                    tag
                }
            },
            Err(e) => {
                panic!("Unable to parse provided file path. Error: {e}")
            }
        }
    }
}
impl TagUtils for Mp3Tag {
    fn artist(&self) -> Option<String> {
        match self.tag.artist() {
            Some(artist) => Some(artist.to_string()),
            _ => None
        }
    }

    fn year(&self) -> Option<String> {
        match &self.tag.year() {
            Some(year) => Some(year.to_string()),
            _ => None
        }
    }
}
#[cfg(test)]
mod test {
    use crate::{get_extension, FileExtension};

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
