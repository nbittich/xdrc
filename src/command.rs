use std::process::Stdio;

use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

use crate::init::ShellCommand;
/// Run a command
pub async fn run(command: &ShellCommand, extra_args: &[String]) {
    let args = [&command.get_args()[..], extra_args].concat();
    let mut cmd = Command::new(command.get_command());
    cmd.args(args);

    cmd.stdout(Stdio::piped());

    let mut child = cmd.spawn().expect("failed to spawn command");

    let stdout = child
        .stdout
        .take()
        .expect("child did not have a handle to stdout");

    let mut reader = BufReader::new(stdout).lines();

    tokio::spawn(async move {
        let _ = child
            .wait()
            .await
            .expect("child process encountered an error");
    });

    while let Some(line) = reader.next_line().await.unwrap() {
        println!("{line}");
    }
}
