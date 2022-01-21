use std::{io::Write, process::Command};

use crate::init::ShellCommand;

pub fn run(command: &ShellCommand, extra_args: Vec<String>) {
    let mut c: Vec<String> = command.get_args().clone();
    c.extend(extra_args);
    let out = Command::new(command.get_command())
        .args(&c[..])
        .output()
        .unwrap();
    std::io::stdout().write_all(&out.stdout[..]).unwrap();
}
