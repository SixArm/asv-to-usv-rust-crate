mod common; use common::*;
use asv_to_usv::examples::*;
use std::process::Command;

#[test]
fn command() {
    let mut command = Command::new(&*COMMAND_OS);
    let actual = command_io_str_to_string(&mut command, EXAMPLE_INPUT_FILES);
    assert_eq!(actual, format!("{}\n", usv::examples::EXAMPLE_FILES_STYLE_SYMBOLS_LAYOUT_RECORDS));
}
