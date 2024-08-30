pub mod cli;
pub mod tags;
pub mod command;
pub mod config;
use anyhow::Result;
use cli::{Args, Parser};
use simple_logger::SimpleLogger;
use command::process::process;
use config::get_config;

fn main() -> Result<()> {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();
    // Get user defined arguments
    let args = Args::parse();
    // Read config.toml props if provided
    let cfg = get_config();
    // Process audio
    process(cfg, &args)?;
    Ok(())
}
