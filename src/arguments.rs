use clap::{AppSettings, Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(global_setting(AppSettings::PropagateVersion))]
#[clap(global_setting(AppSettings::AllowHyphenValues))]
#[clap(global_setting(AppSettings::InferSubcommands))]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Show all available command in this directory
    Info { path: Option<String> },
    /// Add a new command
    Add {
        key: String,
        command: String,
        args: Vec<String>,
    },
    /// Delete a command by key
    Delete { key: String },
    /// Execute a command
    Run {
        key: String,
        extra_args: Vec<String>,
    },
}
