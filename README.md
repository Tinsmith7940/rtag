# audiotag-ci
A simple utility for adding metadata to mp4a and mp3 audio files.

---

## Basic Functionlity
The existing functionality may be expanded in the future, but for now the feature list is:
- Support for both mp4a and mp3 files, with ability to automatically determine file type and process accordingly.
  - In theory any file time that uses the [id3](https://id3.org/Home) standard format may work (wav, aiff, etc) if file extensions are added to the existing list, but they have not been tested.
- Support for writing a few basic tags (basically ones I have a regular use for). This list may grow:
  - title
  - artist
  - year
- Support for clearing all tags off of existing files.
- Ability to define defaults and common combinations of metadata in `profile` inside a config file

## To Execute
- Clone repository
- Execute `cargo build -r`
- Put the binary into your path
- `rtag --help`

## Configuration
To save typing lots of things all the time, defaults can be defined in a central config file. The utility also supports defining common combinations of metadata into profiles that can be referenced from the command line to more
efficiently write tags. Example config below:

`$HOME/.config/rtag/config.toml`
```toml

# When 'clear' is set, the utility will default to clearing all existing metadata before writing new
# Otherwise all existing metadata will be read from the target file, user edits made, and then everthing written back
#
# To set false, simply remove from toml file
clear = true

# Define a profile with common combinations of metadata
[profile.2024]
year = 2024

[profile.podcast]
title = "Audiotag Sessions"
artist = "Audiotag-CI"
year = 2024
```
