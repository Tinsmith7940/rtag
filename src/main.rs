use clap::Parser;
use mp4ameta::*;
use id3::*;
use log::info;

fn main() {
    let args = Args::parse();
    
    if args.file.contains(".mp3") {
        info!("parsing mp3 file");
        let file = id3::Tag::read_from_path(args.file).unwrap();
        let artist = file.artist();
        let year = file.year();
        println!("Artist: {:?}", artist);
        println!("Year: {:?}", year);
    } else {
        info!("parsing m4a file");
        let file = mp4ameta::Tag::read_from_path(args.file).unwrap();

        let artist = file.artist();
        let year = file.year();

        println!("Artist: {:?}", artist);
        println!("Year: {:?}", year);
     
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


