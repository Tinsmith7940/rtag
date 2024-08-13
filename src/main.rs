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
    let mut cmd_list: HashMap<WriteCommands, String> = HashMap::new();

    // Check and see if the user specified a profile to load
    // If yes, then try and load default values from config.toml matching
    // the specified profile.
    if let Some(profile) = args.profile() {
        let cfg_unwrap = cfg.expect("No profile(s) defined in config.toml");
        match cfg_unwrap.profile.get(profile) {
            Some(profile) => {
                for (k, v) in profile.as_table().unwrap().into_iter() {
                    let key = WriteCommands::get_write_command(k.to_string().as_str());
                    cmd_list.insert(key, v.to_string());
                }
            }
            None => {
                log::warn!("No profile matching {profile} found");
            }
        }
    }
    /*
     * Load arguments passed by the user
     * These will always take precedent over any values
     * loaded from the config.toml
     */
    if let Some(title) = args.title() {
        cmd_list.insert(WriteCommands::Title, title.to_string());
    }

    if let Some(year) = args.year() {
        cmd_list.insert(WriteCommands::Year, year.to_string());
    }

    if let Some(artist) = args.artist() {
        cmd_list.insert(WriteCommands::Artist, artist.to_string());
    }

    let audiotagbox = get_audiofile(args.file().clone());

    execute_cmds(cmd_list, audiotagbox)?.write_to_file()?;
    Ok(())
}

fn execute_cmds(
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

#[derive(Eq, Hash, PartialEq, Debug)]
enum WriteCommands {
    Artist,
    Title,
    Year,
}

impl WriteCommands {
    /// Get matching write command
    /// that can be used for audio tag metadata
    /// ```
    /// let cmd = "artist";
    /// let cmd_case_ins = "ArTiSt";
    /// let write_cmd = get_write_command(&cmd);
    /// let write_cmd_ins = get_write_command(&cmd_case_ins);
    ///
    /// assert_eq!(write_cmd, WriteCommands::Artist);
    /// assert_eq!(write_cmd_ins, WriteCommands::Artist);
    /// ```
    ///
    pub fn get_write_command(cmd: &str) -> Self {
        match cmd.to_lowercase().as_str() {
            "artist" => Self::Artist,
            "title" => Self::Title,
            "year" => Self::Year,
            _default => panic!("{cmd} is not a recognized command"),
        }
    }
}

#[derive(Deserialize, Debug)]
struct Config {
    #[allow(dead_code)]
    profile: Map<String, toml::Value>,
}

#[cfg(test)]
pub mod test {

    use crate::WriteCommands;

    #[test]
    fn get_write_cmd() {
        let artist = "artist";
        let title = "title";
        let year = "year";

        assert_eq!(WriteCommands::get_write_command(artist), WriteCommands::Artist);
        assert_eq!(WriteCommands::get_write_command(title), WriteCommands::Title);
        assert_eq!(WriteCommands::get_write_command(year), WriteCommands::Year);
    }

    #[test]
    fn get_write_cmd_case_insensitive() {
        let artist = "aRtiSt";
        let title = "tiTle";
        let year = "yeAr";

        assert_eq!(WriteCommands::get_write_command(artist), WriteCommands::Artist);
        assert_eq!(WriteCommands::get_write_command(title), WriteCommands::Title);
        assert_eq!(WriteCommands::get_write_command(year), WriteCommands::Year);
    }

    #[test]
    #[should_panic(expected = "foo is not a recognized command")]
    fn get_write_cmd_exception_when_no_match() {
        let foo = "foo";
        WriteCommands::get_write_command(foo);
    }
}
