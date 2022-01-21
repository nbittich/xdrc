use serde::{Deserialize, Serialize};
use std::{collections::HashMap, io::Error, path::PathBuf};
use tokio::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct InitFile {
    commands: HashMap<String, ShellCommand>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShellCommand {
    command: String,
    args: Vec<String>,
}

impl ShellCommand {
    pub fn get_command(&self) -> &str {
        &self.command
    }
    pub fn get_args(&self) -> &Vec<String> {
        &self.args
    }
}
impl InitFile {
    async fn get_file() -> tokio::io::Result<String> {
        tokio::fs::read_to_string(InitFile::get_file_path().expect("file path couldn't be fetched"))
            .await
    }
    pub async fn remove(&mut self, key: &str) -> Option<ShellCommand> {
        if let Some(command) = self.commands.remove(key) {
            self.save().await.expect("could not write file");
            Some(command)
        } else {
            None
        }
    }
    pub async fn add(
        &mut self,
        key: String,
        command: String,
        args: Vec<String>,
    ) -> tokio::io::Result<()> {
        self.commands.insert(key, ShellCommand { command, args });
        self.save().await
    }
    fn get_file_path() -> Result<PathBuf, Error> {
        let path = std::env::current_dir()?.as_path().join(".xdrc.json");
        Ok(path)
    }
    pub async fn get() -> InitFile {
        let init_file = InitFile::get_file();
        match init_file.await {
            Ok(f) => {
                let init_f: InitFile = serde_json::from_str(&f).unwrap();
                init_f
            }
            Err(_) => InitFile {
                commands: HashMap::new(),
            },
        }
    }
    pub async fn save(&self) -> tokio::io::Result<()> {
        let input_file_str = serde_json::to_string_pretty(self)?;
        fs::write(InitFile::get_file_path().unwrap(), input_file_str).await
    }
    pub fn get_command(&self, key: &str) -> Option<&ShellCommand> {
        self.commands.get(key)
    }
}
