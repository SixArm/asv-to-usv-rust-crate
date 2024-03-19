#[path = "testing.rs"]
mod testing;
use testing::*;

use std::process::{Command, Stdio};
use std::io::Write;

fn _run(command: &mut Command, stdin: &str) -> String {
    // Spawn child
    let mut child = command
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .spawn()
    .expect("command");

    // Handle stdin
    let child_stdin = child.stdin.as_mut().expect("child_stdin");
    child_stdin.write_all(stdin.as_bytes()).expect("write_all");
    #[allow(dropping_references)]
    let _ = drop(child_stdin);

    // Handle stdout
    let output = child.wait_with_output().expect("wait_with_output");
    String::from_utf8(output.stdout).unwrap()
}

#[test]
fn command() {
    let mut command = Command::new(&*COMMAND_OS);
    let actual = _run(&mut command, &usv::constants::STYLE_CONTROLS_EXAMPLE);
    assert_eq!(actual, format!("{}\n", usv::constants::STYLE_SYMBOLS_EXAMPLE));
}

#[test]
fn command_with_style_braces() {
    let mut command = Command::new(&*COMMAND_OS);
    let actual = _run(command.arg("--style-braces"), &usv::constants::STYLE_CONTROLS_EXAMPLE);
    assert_eq!(actual, format!("{}\n", usv::constants::STYLE_BRACES_EXAMPLE));
}

#[test]
fn command_with_style_controls() {
    let mut command = Command::new(&*COMMAND_OS);
    let actual = _run(command.arg("--style-controls"), &usv::constants::STYLE_CONTROLS_EXAMPLE);
    assert_eq!(actual, format!("{}\n", usv::constants::STYLE_CONTROLS_EXAMPLE));
}

#[test]
fn command_with_style_symbols() {
    let mut command = Command::new(&*COMMAND_OS);
    let actual = _run(command.arg("--style-symbols"), &usv::constants::STYLE_CONTROLS_EXAMPLE);
    assert_eq!(actual, format!("{}\n", usv::constants::STYLE_SYMBOLS_EXAMPLE));
}

#[test]
fn command_with_style_liners() {
    let mut command = Command::new(&*COMMAND_OS);
    let actual = _run(command.arg("--style-liners"), &usv::constants::STYLE_CONTROLS_EXAMPLE);
    assert_eq!(actual, format!("{}\n", usv::constants::STYLE_LINERS_EXAMPLE));
}

#[test]
fn command_with_style_sheets() {
    let mut command = Command::new(&*COMMAND_OS);
    let actual = _run(command.arg("--style-sheets"), &usv::constants::STYLE_CONTROLS_EXAMPLE);
    assert_eq!(actual, format!("{}\n", usv::constants::STYLE_SHEETS_EXAMPLE));
}
