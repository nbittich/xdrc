use std::{io::Write, process::Command};

use crate::init::ShellCommand;

pub fn run(command: &ShellCommand, extra_args: &[String]) {
    let args = [&command.get_args()[..], extra_args].concat();
    let out = Command::new(command.get_command())
        .args(args)
        .output()
        .unwrap();
    std::io::stdout().write_all(&out.stdout[..]).unwrap();
}
