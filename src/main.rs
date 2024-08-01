use clap::Parser;
use id3::*;
use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();

    let args = Args::parse();

    let extension = get_extension(&args.file);

    log::info!("extension: {:?}", extension);
    match extension {
        FileExtension::Mp3 => {
            log::info!("parsing mp3 file");
            let file = id3::Tag::read_from_path(args.file).unwrap();
            let artist = file.artist();
            let year = file.year();
            println!("Artist: {:?}", artist);
            println!("Year: {:?}", year);
        }
        FileExtension::M4a => {
            log::info!("parsing m4a file");
            let file = mp4ameta::Tag::read_from_path(args.file).unwrap();

            let artist = file.artist();
            let year = file.year();

            println!("Artist: {:?}", artist);
            println!("Year: {:?}", year);
        }
    }
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
    M4a,
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
