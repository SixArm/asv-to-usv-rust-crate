//! # asv-to-usv
//! 
//! Convert [ASCII Separated Values (ASV)](https://github.com/SixArm/usv/tree/main/doc/comparisons/asv)
//! to [Unicode Separated Values (USV)](https://github.com/sixarm/usv).
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
//! cat example.asv | asv-to-usv
//! ```
//! 
//! More examples below.
//! 
//! ## Options
//! 
//! Options for USV separators and modifiers:
//! 
//! * -u, --unit-separator : Set the unit separator (US) string.
//! 
//! * -r, --record-separator : Set the record separator (RS) string.
//! 
//! * -g, --group-separator : Set the group separator (GS) string.
//! 
//! * -f, --file-separator : Set the file separator (FS) string.
//! 
//! * -e, --escape : Set the escape (ESC) string.
//! 
//! * -z, --end-of-transmission : Set the end of transmission (EOT) string.
//! 
//! Options for USV style:
//! 
//! * --style-braces : Set the style to use braces, such as "{US}" for Unit Separator.
//! 
//! * --style-controls : Set the style to use controls, such as "\u001F" for Unit Separator.
//! 
//! * --style-symbols : Set the style to use symbols, such as "␟" for Unit Separator.
//! 
//! Options for USV layout:
//! 
//! * --layout-0: Show each item with no line around it. This is no layout, in other words one long line.
//! 
//! * --layout-1: Show each item with one line around it. This is like single-space lines for long form text.
//! 
//! * --layout-2: Show each item with two lines around it. This is like double-space lines for long form text.
//! 
//! * --layout-units: Show each unit on one line. This can be helpful for line-oriented tools.
//! 
//! * --layout-records: Show each record on one line. This is like a typical spreadsheet sheet export.
//! 
//! * --layout-groups: Show each group on one line. This can be helpful for folio-oriented tools.
//! 
//! * --layout-files: Show one file on one line. This can be helpful for archive-oriented tools.
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
//! ## Example
//! 
//! 
//! Suppose example.asv contains:
//! 
//! ```asv
//! a\u001Fb\u001Ec\u001Fd\u001E
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
//! ```usv
//! a␟b␟␞
//! c␟d␟␞
//! ```
//! 
//! If you prefer to render markers with braces, to see the markers more easily:
//! 
//! ```sh
//! cat example.csv | asv-to-usv --style-braces
//! ```
//! 
//! Output:
//! 
//! ```usv
//! a{US}b{US}{RS}
//! c{US}d{US}{RS}
//! ```
//! 
//! ## FAQ
//! 
//! ### What converters are available?
//!
//! * [asv-to-usv](https://crates.io/crates/asv-to-usv) & [usv-to-asv](https://crates.io/crates/usv-to-asv)
//!
//! * [csv-to-usv](https://crates.io/crates/asv-to-csv) & [usv-to-csv](https://crates.io/crates/usv-to-csv)
//!
//! * [json-to-usv](https://crates.io/crates/json-to-usv) & [usv-to-json](https://crates.io/crates/usv-to-json)
//!
//! * [xlsx-to-usv](https://crates.io/crates/xlsx-to-usv) & [usv-to-xlsx](https://crates.io/crates/usv-to-xlsx)
//!
//! ### When to use this command?
//! 
//! Use this command when you want to convert from ASV to USV.
//! 
//! A typical use case is when you have ASV data, such as a collection of units and
//! records, and you want to convert it to USV data, such as for visual editing.
//! 
//! Our real-world use case is converting a bunch of document-oriented data from a
//! variety of programs to a variety of data formats, spreadsheets, and databases.
//! 
//! ### Why use USV instead of ASV?
//! 
//! See the documentation for [USV](https://github.com/sixarm/usv).
//! 
//! ### Is USV aiming to become a standard?
//! 
//! Yes, USV is submitted to IETF.org as an Internet-Draft work in progress:
//! [link](https://datatracker.ietf.org/doc/draft-unicode-separated-values/).
//! 
//! ### Can I build my own USV tools?
//!
//! Yes, and you may freely use the
//! [USV Rust crate](https://github.com/sixarm/usv-rust-crate).
//!
//! ## Help wanted
//! 
//! Constructive feedback welcome. Pull requests and feature requests welcome.
//! 
//! ## Tracking
//! 
//! * Package: asv-to-usv-rust-crate
//! * Version: 1.5.2
//! * Created: 2024-03-09T13:33:20Z
//! * Updated: 2024-04-10T20:51:41Z
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
    let usv = asv_to_usv(&s, &args.style);
    println!("{}", &usv);
    Ok(())
}
