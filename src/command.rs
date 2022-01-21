use tokio::io::{self, AsyncWriteExt};
use tokio::process::Command;

use crate::init::ShellCommand;

pub async fn run(command: &ShellCommand, extra_args: &[String]) {
    let args = [&command.get_args()[..], extra_args].concat();
    let out = Command::new(command.get_command())
        .args(args)
        .output()
        .await
        .unwrap();
    let mut stdout = io::stdout();
    stdout.write_all(&out.stdout[..]).await.unwrap();
}
