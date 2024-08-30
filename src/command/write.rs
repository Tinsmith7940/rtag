#[derive(Eq, Hash, PartialEq, Debug)]
pub enum WriteCommands {
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
    pub fn get_write_command(cmd: &str) -> Option<Self> {
        match cmd.to_lowercase().as_str() {
            "artist" => Some(Self::Artist),
            "title" => Some(Self::Title),
            "year" => Some(Self::Year),
            _default => None,
        }
    }
}

#[cfg(test)]
pub mod test {

    use super::WriteCommands;

    #[test]
    fn get_write_cmd() {
        let artist = "artist";
        let title = "title";
        let year = "year";

        assert_eq!(WriteCommands::get_write_command(artist).unwrap(), WriteCommands::Artist);
        assert_eq!(WriteCommands::get_write_command(title).unwrap(), WriteCommands::Title);
        assert_eq!(WriteCommands::get_write_command(year).unwrap(), WriteCommands::Year);
    }

    #[test]
    fn get_write_cmd_case_insensitive() {
        let artist = "aRtiSt";
        let title = "tiTle";
        let year = "yeAr";

        assert_eq!(WriteCommands::get_write_command(artist).unwrap(), WriteCommands::Artist);
        assert_eq!(WriteCommands::get_write_command(title).unwrap(), WriteCommands::Title);
        assert_eq!(WriteCommands::get_write_command(year).unwrap(), WriteCommands::Year);
    }

    #[test]
    fn get_none_when_no_match() {
        let foo = "foo";
        assert!(WriteCommands::get_write_command(foo).is_none());
    }
}
