pub mod cli;
pub mod tags;
use anyhow::Result;
use cli::{Args, Parser};
use serde::Deserialize;
use simple_logger::SimpleLogger;
use std::collections::HashMap;
use std::fs;
use tags::{get_audiofile, TagUtils};
use toml::{self, map::Map};

fn main() -> Result<()> {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();

    let args = Args::parse();

    let cfg = get_config();
    execute(cfg, &args)?;

    Ok(())
}

fn execute(cfg: Option<Config>, args: &Args) -> Result<()> {
    let mut preproc_cmd_list: Vec<PreProcessCommands> = vec![];
    let mut cmd_list: HashMap<WriteCommands, String> = HashMap::new();

    if let Some(profile) = args.profile() {
        let cfg_unwrap = cfg.expect("No profile(s) defined in config.toml");
        match cfg_unwrap.profile.get(profile) {
            Some(profile) => {
                for (k, v) in profile.as_table().unwrap().into_iter() {
                    if let Some(key) = WriteCommands::get_write_command(k.to_string().as_str()) {
                        cmd_list.insert(key, v.to_string());
                    } else if let Some(pre_key) = PreProcessCommands::get_preprocess_command(k.to_string().as_str()) {
                        preproc_cmd_list.push(pre_key);
                    } else {
                        log::warn!("Command {k} from profile {profile} is not a valid command. Skipping...");
                    }
                }
            }
            None => {
                log::warn!("No profile matching {profile} found");
            }
        }
    }

    if *args.clear() && !preproc_cmd_list.contains(&PreProcessCommands::Clear)  {
            preproc_cmd_list.push(PreProcessCommands::Clear);
    }

    if let Some(title) = args.title() {
        cmd_list.insert(WriteCommands::Title, title.to_string());
    }

    if let Some(year) = args.year() {
        cmd_list.insert(WriteCommands::Year, year.to_string());
    }

    if let Some(artist) = args.artist() {
        cmd_list.insert(WriteCommands::Artist, artist.to_string());
    }

    let init_audiotagbox = get_audiofile(args.file().clone());

    execute_write_cmds(
        cmd_list,
        execute_preproc_cmds(preproc_cmd_list, init_audiotagbox)?
    )?
    .write_to_file()?;
    Ok(())
}

fn execute_preproc_cmds(
    cmds: Vec<PreProcessCommands>,
    mut audiotagbox: Box<dyn TagUtils>,
) -> Result<Box<dyn TagUtils>> {
    for k in cmds {
        match k {
            PreProcessCommands::Clear => audiotagbox.clear()?,
        }
    }
    Ok(audiotagbox)
}

fn execute_write_cmds(
    cmds: HashMap<WriteCommands, String>,
    mut audiotagbox: Box<dyn TagUtils>,
) -> Result<Box<dyn TagUtils>> {
    for (k, v) in cmds {
        match k {
            WriteCommands::Title => audiotagbox.set_title(v),
            WriteCommands::Year => audiotagbox.set_year(v),
            WriteCommands::Artist => audiotagbox.set_artist(v),
        }
    }
    Ok(audiotagbox)
}

fn get_config() -> Option<Config> {
    if let Ok(file_contents) = fs::read_to_string("config.toml") {
        match toml::from_str(&file_contents) {
            Ok(cnf) => {
                return Some(cnf);
            }
            Err(e) => {
                log::warn!("Unable to successfully parse toml config: {e}");
                return None;
            }
        }
    }
    None
}

/// Commands that should always be executed before WriteCommands
#[derive(Eq, Hash, PartialEq)]
enum PreProcessCommands {
    /// Clear all metadata from existing tag
    Clear,
}

impl PreProcessCommands {
    fn get_preprocess_command(cmd: &str) -> Option<Self> {
        match cmd.to_lowercase().as_str() {
            "clear" => Some(Self::Clear),
            _default => None,
        }
    }
}

#[derive(Eq, Hash, PartialEq)]
enum WriteCommands {
    Artist,
    Title,
    Year,
}

impl WriteCommands {
    fn get_write_command(cmd: &str) -> Option<Self> {
        match cmd.to_lowercase().as_str() {
            "artist" => Some(Self::Artist),
            "title" => Some(Self::Title),
            "year" => Some(Self::Year),
            _default => None,
        }
    }
}

#[derive(Deserialize, Debug)]
struct Config {
    #[allow(dead_code)]
    profile: Map<String, toml::Value>,
}
