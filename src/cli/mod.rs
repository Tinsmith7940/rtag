pub use clap::Parser;

/// A simple program to read and write audio file tags
#[derive(Parser, Debug)]
#[command(version, about, long_about = "None")]
pub struct Args {
    /// target audio file
    #[arg(short, long)]
    file: String,

    /// artist
    #[arg(short, long)]
    artist: Option<String>,

    /// Year
    #[arg(short, long)]
    year: Option<i32>,

    /// Title
    #[arg(short, long)]
    title: Option<String>
}

impl Args {
    pub fn file(&self) -> &String {
        &self.file
    }

    pub fn artist(&self) -> &Option<String> {
        &self.artist
    }

    pub fn year(&self) -> &Option<i32> {
        &self.year
    }

    pub fn title(&self) -> &Option<String> {
        &self.title
    }
}
