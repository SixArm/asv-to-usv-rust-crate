#[path = "testing.rs"]
mod testing;
use testing::*;

use std::process::{Command, Stdio};
use std::io::Write;

// use std::path::PathBuf;
// use once_cell::sync::Lazy;
//
// pub static DIR: Lazy<PathBuf> = Lazy::new(||
//     TESTS_DIR.join("command")
// );

#[test]
fn command() {
    let input = String::from("a\u{001F}b\u{001F}\u{001E}c\u{001F}d\u{001F}\u{001E}\u{001D}e\u{001F}f\u{001F}\u{001E}g\u{001F}h\u{001F}\u{001E}\u{001D}\u{001C}i\u{001F}j\u{001F}\u{001E}k\u{001F}l\u{001F}\u{001E}\u{001D}m\u{001F}n\u{001F}\u{001E}o\u{001F}p\u{001F}\u{001E}\u{001D}\u{001C}");
    let expect = "a␟b␟␞c␟d␟␞␝e␟f␟␞g␟h␟␞␝␜i␟j␟␞k␟l␟␞␝m␟n␟␞o␟p␟␞␝␜\n";

    // Given
    let mut command = Command::new(&*COMMAND_OS)
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .spawn()
    .expect("command");

    // When
    let child_stdin = command.stdin.as_mut().expect("child_stdin");
    child_stdin.write_all(input.as_bytes()).expect("write_all");
    #[allow(dropping_references)]
    let _ = drop(child_stdin);

    // Then
    let output = command.wait_with_output().expect("wait_with_output");
    let actual: String = String::from_utf8(output.stdout).unwrap();
    assert_eq!(actual, expect);
}
