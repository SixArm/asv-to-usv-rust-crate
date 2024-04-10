mod common; use common::*;
use asv_to_usv::examples::*;
use std::process::Command;

#[test]
fn command_with_style_symbols() {
    let mut command = Command::new(&*COMMAND_OS);
    let actual = command_io_str_to_string(command.arg("--style-symbols"), EXAMPLE_ASV_FILES);
    assert_eq!(actual, format!("{}\n", usv::examples::EXAMPLE_FILES_STYLE_SYMBOLS_LAYOUT_RECORDS));
}

#[test]
fn command_with_style_controls() {
    let mut command = Command::new(&*COMMAND_OS);
    let actual = command_io_str_to_string(command.arg("--style-controls"), EXAMPLE_ASV_FILES);
    assert_eq!(actual, format!("{}\n", usv::examples::EXAMPLE_FILES_STYLE_CONTROLS_LAYOUT_RECORDS));
}

#[test]
fn command_with_style_braces() {
    let mut command = Command::new(&*COMMAND_OS);
    let actual = command_io_str_to_string(command.arg("--style-braces"), EXAMPLE_ASV_FILES);
    assert_eq!(actual, format!("{}\n", usv::examples::EXAMPLE_FILES_STYLE_BRACES_LAYOUT_RECORDS));
}
