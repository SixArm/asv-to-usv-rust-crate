#[path = "testing.rs"]
mod testing;
use testing::*;

#[path = "command_io.rs"]
mod command_io;
use command_io::*;

use std::process::Command;

#[test]
fn command_with_style_braces() {
    let mut command = Command::new(&*COMMAND_OS);
    let actual = command_io(command.arg("--style-braces"), &usv::constants::STYLE_CONTROLS_EXAMPLE);
    assert_eq!(actual, format!("{}\n", usv::constants::STYLE_BRACES_EXAMPLE));
}

#[test]
fn command_with_style_controls() {
    let mut command = Command::new(&*COMMAND_OS);
    let actual = command_io(command.arg("--style-controls"), &usv::constants::STYLE_CONTROLS_EXAMPLE);
    assert_eq!(actual, format!("{}\n", usv::constants::STYLE_CONTROLS_EXAMPLE));
}

#[test]
fn command_with_style_symbols() {
    let mut command = Command::new(&*COMMAND_OS);
    let actual = command_io(command.arg("--style-symbols"), &usv::constants::STYLE_CONTROLS_EXAMPLE);
    assert_eq!(actual, format!("{}\n", usv::constants::STYLE_SYMBOLS_EXAMPLE));
}

#[test]
fn command_with_style_liners() {
    let mut command = Command::new(&*COMMAND_OS);
    let actual = command_io(command.arg("--style-liners"), &usv::constants::STYLE_CONTROLS_EXAMPLE);
    assert_eq!(actual, format!("{}\n", usv::constants::STYLE_LINERS_EXAMPLE));
}

#[test]
fn command_with_style_sheets() {
    let mut command = Command::new(&*COMMAND_OS);
    let actual = command_io(command.arg("--style-sheets"), &usv::constants::STYLE_CONTROLS_EXAMPLE);
    assert_eq!(actual, format!("{}\n", usv::constants::STYLE_SHEETS_EXAMPLE));
}
