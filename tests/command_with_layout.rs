mod common; use common::*;
use asv_to_usv::examples::*;
use std::process::Command;

#[test]
fn command_with_layout_0() {
    let mut command = Command::new(&*COMMAND_OS);
    let actual = command_io_str_to_string(command.arg("--layout-0"), EXAMPLE_INPUT_FILES);
    assert_eq!(actual, format!("{}\n", usv::examples::EXAMPLE_FILES_STYLE_SYMBOLS_LAYOUT_0));
}

#[test]
fn command_with_layout_1() {
    let mut command = Command::new(&*COMMAND_OS);
    let actual = command_io_str_to_string(command.arg("--layout-1"), EXAMPLE_INPUT_FILES);
    assert_eq!(actual, format!("{}\n", usv::examples::EXAMPLE_FILES_STYLE_SYMBOLS_LAYOUT_1));
}

#[test]
fn command_with_layout_2() {
    let mut command = Command::new(&*COMMAND_OS);
    let actual = command_io_str_to_string(command.arg("--layout-2"), EXAMPLE_INPUT_FILES);
    assert_eq!(actual, format!("{}\n", usv::examples::EXAMPLE_FILES_STYLE_SYMBOLS_LAYOUT_2));
}

#[test]
fn command_with_layout_units() {
    let mut command = Command::new(&*COMMAND_OS);
    let actual = command_io_str_to_string(command.arg("--layout-units"), EXAMPLE_INPUT_FILES);
    assert_eq!(actual, format!("{}\n", usv::examples::EXAMPLE_FILES_STYLE_SYMBOLS_LAYOUT_UNITS));
}

#[test]
fn command_with_layout_records() {
    let mut command = Command::new(&*COMMAND_OS);
    let actual = command_io_str_to_string(command.arg("--layout-records"), EXAMPLE_INPUT_FILES);
    assert_eq!(actual, format!("{}\n", usv::examples::EXAMPLE_FILES_STYLE_SYMBOLS_LAYOUT_RECORDS));
}

#[test]
fn command_with_layout_groups() {
    let mut command = Command::new(&*COMMAND_OS);
    let actual = command_io_str_to_string(command.arg("--layout-groups"), EXAMPLE_INPUT_FILES);
    assert_eq!(actual, format!("{}\n", usv::examples::EXAMPLE_FILES_STYLE_SYMBOLS_LAYOUT_GROUPS));
}
