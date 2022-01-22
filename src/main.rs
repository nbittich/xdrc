use clap::Parser;
use tokio::io;
use xdrc::arguments;
use xdrc::init;

#[tokio::main]
async fn main() -> io::Result<()> {
    let cli = arguments::Cli::parse();

    match &cli.command {
        arguments::Commands::Info { path } => {
            println!("'init' was used, name is: {:?}", path);
            let init_file = init::InitFile::get().await;
            println!("{:?}", init_file);
        }
        arguments::Commands::Add { key, command, args } => {
            println!(
                "'add was used', key is {:?}, command is {:?}, args is {:?}",
                key, command, args
            );
            let mut init_file = init::InitFile::get().await;
            init_file
                .add(key.to_string(), command.to_string(), args.to_owned())
                .await?;
        }
        arguments::Commands::Run { key, extra_args } => {
            println!(
                "'run' was used, key is: {:?}, extra_args: {:?}",
                key, extra_args
            );
            let init_file = init::InitFile::get().await;
            if let Some(sc) = init_file.get_command(key) {
                xdrc::command::run(sc, extra_args).await;
            }
        }
        arguments::Commands::Delete { key } => {
            println!("remove was used with key {key}");
            let mut init_file = init::InitFile::get().await;
            println!(
                "remove shellcommand: {}",
                init_file.remove(key).await.is_some()
            );
        }
    }
    Ok(())
}
