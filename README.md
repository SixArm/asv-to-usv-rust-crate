# asv-to-usv

Convert ASCII Separated Values (ASV) to [Unicode Separated Values (USV)](https://github.com/sixarm/usv).

Syntax:

```sh
stdin | asv-to-usv | stdout
```

Example:

```sh
cat example.asv | asv-to-usv
```

Example with output to a file:

```sh
cat example.asv | asv-to-usv > example.usv
```

## Options

* -h, --help : Print help

* -V, --version : Print version

* -v, --verbose... : Set the verbosity level: 0=none, 1=error, 2=warn, 3=info, 4=debug, 5=trace. Example: --verbose …

* --test : Print test output for debugging, verifying, tracing, and the like. Example: --test


## Install

Install:

```sh
cargo install asv-to-usv
```

Link: [https://crates.io/crates/asv-to-usv](https://crates.io/crates/asv-to-usv)


## Example

Suppose example.asv contains:

```usv
a\u{001F}b\u{001F}c\u{001F}\u{001E}
d\u{001F}e\u{001F}f\u{001F}\u{001E}
g\u{001F}h\u{001F}i\u{001F}\u{001E}
```

Run:

```sh
cat example.usv | asv-to-usv
```

Output:

```asv
a␟b␟c␟␞
d␟e␟f␟␞
g␟h␟i␟␞
```

## FAQ

### When to use this command?

Use this command when you want to convert from ASV to USV.

A typical use case is when you have ASV data, such as a collection of units and
records, and you want to convert it to USV data, such as for visual editing.

Our real-world use case is converting a bunch of document-oriented data from a
variety of programs to a variety of data formats, spreadsheets, and databases.

### Is there a similar command to convert from USV to ASV?

Yes: [asv-to-usv](https://crates.io/crates/usv-to-asv).

### Why use USV instead of ASV?

See the documentation for [USV](https://github.com/sixarm/usv).

### Is USV aiming to become a standard?

Yes, USV is submitted to IETF.org as an Internet-Draft work in progress:
[link](https://datatracker.ietf.org/doc/draft-unicode-separated-values/).

## Help wanted

Constructive feedback welcome. Pull requests and feature requests welcome.

## Tracking

* Package: asv-to-usv-rust-crate
* Version: 1.0.0
* Created: 2024-03-09T13:33:20Z
* Updated: 2024-03-12T12:55:19Z
* License: MIT or Apache-2.0 or GPL-2.0 or GPL-3.0 or contact us for more
* Contact: Joel Parker Henderson (joel@sixarm.com)
