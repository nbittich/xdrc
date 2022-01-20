use std::{ io::Error, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct InitFile {
    commands: Vec<ShellCommand>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShellCommand {
    key: String,
    command: String,
    args: Vec<String>,
}

impl InitFile {
    fn get_file() -> Result<String, Error> {
        std::fs::read_to_string(InitFile::get_file_path()?)
    }
    pub fn add(&mut self, key: String, command: String, args: Vec<String>) -> Result<(), Error> {
        self.commands.push(ShellCommand {
            key, command, args
        });
        // todo make that better
        self.save()
    }
   fn get_file_path() -> Result<PathBuf, Error> {
        let path = std::env::current_dir()?.as_path().join(".xdrc.json");
        Ok(path)
    }
    pub fn get() -> InitFile {
        let init_file = InitFile::get_file();
        match init_file {
            Ok(f) => {
                let init_f: InitFile = serde_json::from_str(&f).unwrap();
                init_f
            }
            Err(_) => InitFile { commands: vec![] },
        }
    }
    pub fn save(&self) -> Result<(), Error> {
        let input_file_str = serde_json::to_string_pretty(self)?;
        std::fs::write(InitFile::get_file_path().unwrap(), input_file_str)
    }
}
