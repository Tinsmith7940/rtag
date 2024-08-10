pub mod cli;
pub mod tags;
use tags::get_audiofile;
use simple_logger::SimpleLogger;
use cli::{ Args, Parser };
use anyhow::Result;

fn main() -> Result<()> {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();

    let args = Args::parse();

    let mut audiotagbox = get_audiofile(args.file().clone());

    if let Some(title) = args.title() {
        audiotagbox.set_title(title.to_string());
    }

    if let Some(year) = args.year() {
        audiotagbox.set_year(*year);
    }

    if let Some(artist) = args.artist() {
        audiotagbox.set_artist(artist.to_string());
    }

    audiotagbox.write_to_file()?;

    Ok(())
}
