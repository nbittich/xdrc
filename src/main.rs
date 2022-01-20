use clap::Parser;
use std::process::Command;
use xdrc::arguments;
use xdrc::init;
fn main() {
    let out = Command::new("docker-compose")
        .args(["-version"])
        .output()
        .unwrap();
    let res: String = out
        .stdout
        .into_iter()
        .map(|c| c as char)
        .filter(|s| s.ne(&'\n'))
        .collect();
    println!("{:?}", res);

    let cli = arguments::Cli::parse();

    match &cli.command {
        arguments::Commands::Init { path } => {
            println!("'init' was used, name is: {:?}", path);
            let init_file = init::InitFile::get();
            println!("{:?}", init_file);
        }
        arguments::Commands::Add { key, command, args } => {
            println!(
                "'add was used', key is {:?}, command is {:?}, args is {:?}",
                key, command, args
            );
            let mut init_file = init::InitFile::get();
            init_file.add(key.to_string(), command.to_string(), args.to_owned()).expect("Could not add command");

        }
        arguments::Commands::Run { key, extra_args } => {
            println!(
                "'run' was used, key is: {:?}, extra_args: {:?}",
                key, extra_args
            )
        }
    }
}
