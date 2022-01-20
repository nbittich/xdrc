use clap::{AppSettings, Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(global_setting(AppSettings::PropagateVersion))]
#[clap(global_setting(AppSettings::AllowHyphenValues))]
#[clap(global_setting(AppSettings::UseLongFormatForHelpSubcommand))]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Init xdrc
    Init { path: Option<String> },
    /// Add new command
    Add {
        key: String,
        command: String,
        args: Vec<String>,
    },
    /// Run command
    Run {
        key: String,
        extra_args: Vec<String>,
    },
}
