use crate::cli::Args;
use crate::command::write::WriteCommands;
use std::collections::HashMap;
use crate::tags::{get_audiofile, TagUtils};
use crate::config::Config;
use anyhow::Result;

pub fn process(cfg: Option<Config>, args: &Args) -> Result<()> {
    let mut cmd_list: HashMap<WriteCommands, String> = HashMap::new();
    let cfg_unwrap = cfg.expect("config.toml could not be parsed");

    // Check and see if the user specified a profile to load
    // If yes, then try and load default values from config.toml matching
    // the specified profile.
    if let Some(profile) = args.profile() {
        match cfg_unwrap.profile.get(profile) {
            Some(profile) => {
                for (k, v) in profile.as_table().unwrap().into_iter() {
                    if let Some(key) = WriteCommands::get_write_command(k.to_string().as_str()) {
                        cmd_list.insert(key, v.to_string());
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

    // Determine if user wants a 'clean' tag, or would rather load existing
    // metadata from the target file before editing/adding additional data and writing back
    //
    // Check for user specified 'clear' command first, and then in the event false is passed in,
    // check the existing config file for any default preference.
    let clean_tag = *args.clear() || cfg_unwrap.clear.unwrap_or(false);

    let audiotagbox = get_audiofile(args.file().clone(), clean_tag);

    execute_write_cmds(
        cmd_list,
        audiotagbox
    )?
    .write_to_file()?;
    Ok(())
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

