use clap::Parser;
use tokio::io;
use xdrc::arguments;
use xdrc::init;

#[tokio::main]
async fn main() -> io::Result<()> {
    let cli = arguments::Cli::parse();

    match &cli.command {
        arguments::Commands::Info { path: _ } => {
            let init_file = init::InitFile::get().await;
            println!("{}", serde_json::to_string_pretty(&init_file).unwrap());
        }
        arguments::Commands::Add { key, command, args } => {
            let mut init_file = init::InitFile::get().await;
            init_file
                .add(key.to_string(), command.to_string(), args.to_owned())
                .await?;
        }
        arguments::Commands::Run { key, extra_args } => {
            let init_file = init::InitFile::get().await;
            if let Some(sc) = init_file.get_command(key) {
                xdrc::command::run(sc, extra_args).await;
            }
        }
        arguments::Commands::Delete { key } => {
            let mut init_file = init::InitFile::get().await;
            println!(
                "remove command: {}",
                init_file.remove(key).await.is_some()
            );
        }
    }
    Ok(())
}
