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

    pub fn artist(&self) -> String {
        self.artist.clone().unwrap_or("None".to_string())
    }

    pub fn year(&self) -> i32 {
        self.year.unwrap_or(0)
    }

    pub fn title(&self) -> String {
        self.title.clone().unwrap_or("None".to_string())
    }
}
