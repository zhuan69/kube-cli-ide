use std::os::unix::process::CommandExt;
use std::process::{Child, Command, Output, Stdio};

use serde::Deserialize;

pub fn run_command(command: String, args: Vec<String>) -> Output {
    let mut c = Command::new(command);
    c.args(args);
    c.output().expect("failed to exec command")
}
pub fn run_command_child_process(command: String, args: Vec<String>) -> Child {
    Command::new(command)
        .args(args)
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to start child process")
}

pub fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

pub fn serde_string_to_json<'a, T>(json_string: &'a str) -> Option<T>
where
    T: Deserialize<'a>,
{
    match serde_json::from_str(json_string) {
        Ok(success) => Some(success),
        Err(err) => {
            eprintln!("error while parse data with error: {:?}", err);
            None
        }
    }
}
