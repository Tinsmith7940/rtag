pub mod cli;
pub mod tags;
use tags::get_audiofile;
use simple_logger::SimpleLogger;
use cli::{ Args, Parser };

fn main() {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();

    let args = Args::parse();

    let audiotagbox = get_audiofile(args.file().clone())
        .get_tag()
        .unwrap_or_else(|| {
            panic!("Unable to parse tags from audiofile");
        });

    println!("Artist: {:?}", audiotagbox.artist());
    println!("Year: {:?}", audiotagbox.year());
    println!("Title: {:?}", audiotagbox.title());
}
