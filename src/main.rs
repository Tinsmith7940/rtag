pub mod cli;
pub mod command;
pub mod config;
pub mod tags;
use anyhow::Result;
use cli::{Args, Parser};
use command::process::process;
use config::get_config;
use simple_logger::SimpleLogger;

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
