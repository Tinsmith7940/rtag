pub use clap::Parser;

/// A simple program to read and write audio file tags
#[derive(Parser, Debug)]
#[command(version, about, long_about = "None")]
pub struct Args {
    /// target audio file
    #[arg(short, long)]
    file: String,

    /// Artist
    #[arg(short, long)]
    artist: Option<String>,

    /// Year
    #[arg(short, long)]
    year: Option<i32>,

    /// Title
    #[arg(short, long)]
    title: Option<String>,

    /// Name of config profile to use for default values
    /// This is defined in the utilities config.toml
    #[arg(long)]
    profile: Option<String>,

    #[arg(long)]
    clear: bool,
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

    pub fn profile(&self) -> &Option<String> {
        &self.profile
    }

    pub fn clear(&self) -> &bool {
        &self.clear
    }
}
