use clap::Parser;
use mp4ameta::*;

fn main() {
    let args = Args::parse();
    let file = mp4ameta::Tag::read_from_path(args.file).unwrap();

    let artist = file.artist();
    let year = file.year();

    println!("Artist: {:?}", artist);
    println!("Year: {:?}", year);
}

/// A simple program to read and write audio file tags
#[derive(Parser, Debug)]
#[command(version, about, long_about = "None")]
pub struct Args {

    /// target audio file
    #[arg(short, long)]
    file: String,
}


