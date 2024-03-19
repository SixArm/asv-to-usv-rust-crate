//! # asv-to-usv
//!
//! Convert [Unicode Separated Values (USV)](https://github.com/sixarm/usv) to ASCII Separated Values (ASV).
//!
//! Syntax:
//!
//! ```sh
//! stdin | asv-to-usv | stdout
//! ```
//!
//! Example:
//!
//! ```sh
//! cat example.usv | asv-to-usv
//! ```
//!
//! Example with output to a file:
//!
//! ```sh
//! cat example.usv | asv-to-usv > example.asv
//! ```
//! ## Options
//! 
//! Options for USV separators and modifiers:
//! 
//! * -u | --unit-separator : Set the unit separator string.
//! 
//! * -r | --record-separator : Set the record separator string.
//! 
//! * -g | --group-separator : Set the group separator string.
//! 
//! * -f | --file-separator : Set the file separator string.
//! 
//! * --escape : Set the escape string.
//! 
//! * --end-of-transmission : Set the end-of-transmission string.
//! 
//! Options for USV style sets:
//! 
//! * --style-braces : Set the style to use braces, such as "{US}" for Unit Separator.
//! 
//! * --style-controls : Set the style to use controls, such as "\u{001F}" for Unit Separator.
//! 
//! * --style-symbols : Set the style to use symbols, such as "␟" for Unit Separator.
//! 
//! * --style-liners : Set the style to use liners wrapping every symbol, such as "\n␟\n" for Unit Separator.
//! 
//! * --style-sheets : Set the style similar to spreadsheet sheets, such as "␟" for Unit Separator and "␟\n" for Record Separator.
//! 
//! Options for command line tools:
//! 
//! * -h, --help : Print help
//! 
//! * -V, --version : Print version
//! 
//! * -v, --verbose... : Set the verbosity level: 0=none, 1=error, 2=warn, 3=info, 4=debug, 5=trace. Example: --verbose …
//! 
//! * --test : Print test output for debugging, verifying, tracing, and the like. Example: --test
//! 
//! ## Install
//! 
//! Install:
//! 
//! ```sh
//! cargo install asv-to-usv
//! ```
//! 
//! Link: [https://crates.io/crates/asv-to-usv](https://crates.io/crates/asv-to-usv)
//! 
//! 
//! ## Example
//! 
//! Suppose example.asv contains:
//! 
//! ```usv
//! a\u{001F}b\u{001F}\u{001E}
//! c\u{001F}d\u{001F}\u{001E}
//! ```
//! 
//! Run:
//! 
//! ```sh
//! cat example.asv | asv-to-usv
//! ```
//! 
//! Output:
//! 
//! ```asv
//! a␟b␟␞
//! c␟d␟␞
//! ```
//! 
//! ## FAQ
//!
//! ### When to use this command?
//!
//! Use this command when you want to convert from USV to ASV.
//!
//! A typical use case is when you have ASV data, such as a collection of units and
//! records, and you want to convert it to USV data, such as for visual editing.
//!
//! Our real-world use case is converting a bunch of USV document-oriented data
//! from a variety of programs, including a CMS, to USV so we're better-able to
//! import the data into Excel.
//!
//! ### Is there a similar command to convert from ASV to USV?
//!
//! Yes: [usv-to-asv](https://crates.io/crates/usv-to-asv).
//!
//! ### Why use USV instead of ASV?
//!
//! See the documentation for [USV](https://github.com/sixarm/usv).
//!
//! ### Is USV aiming to become a standard?
//!
//! Yes and we've submitted the first draft of the USV standard to the IETF:
//! [link](https://datatracker.ietf.org/doc/draft-unicode-separated-values/).
//!
//! ## Help wanted
//!
//! Constructive feedback welcome. Pull requests and feature requests welcome.
//!
//! ## Tracking
//!
//! * Package: asv-to-usv-rust-crate
//! * Version: 1.2.0
//! * Created: 2024-03-09T13:33:20Z
//! * Updated: 2024-03-19T16:25:30Z
//! * License: MIT or Apache-2.0 or GPL-2.0 or GPL-3.0 or contact us for more
//! * Contact: Joel Parker Henderson (joel@sixarm.com)

//// log
#[macro_use]
extern crate log;
extern crate env_logger;

use asv_to_usv::asv_to_usv;
use std::io::{Read, stdin};

pub mod app {
    pub mod args;
    pub mod clap;
    pub mod log;
}

fn main() -> std::io::Result<()> {
    let args: crate::app::args::Args = crate::app::clap::clap();
    if args.test { println!("{:?}", args); }
    let mut stdin = stdin().lock();
    let mut s = String::new();
    stdin.read_to_string(&mut s)?;
    println!("{}", asv_to_usv(&s, &args.style));
    Ok(())
}
