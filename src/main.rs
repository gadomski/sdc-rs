//! Executable for working with .sdc files.

extern crate docopt;
extern crate rustc_serialize;
extern crate sdc;

use std::process::exit;

use docopt::Docopt;

use sdc::Reader;

const USAGE: &'static str = "
Work with .sdc files.

Usage:
    sdc info <infile> [--brief]
    \
                             sdc (--help | --version)

Options:
    -h --help       Show this \
                             screen.
    --version       Show version.
    --brief         Only \
                             display information from the header.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_infile: String,
    flag_brief: bool,
    flag_help: bool,
    flag_version: bool,
    cmd_info: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                         .and_then(|d| {
                             d.version(Some(env!("CARGO_PKG_VERSION").to_string()))
                              .help(true)
                              .decode()
                         })
                         .unwrap_or_else(|e| e.exit());

    if args.cmd_info {
        let reader = match Reader::from_path(&args.arg_infile) {
            Ok(reader) => reader,
            Err(err) => {
                println!("ERROR: unable to create reader for {}: {}",
                         args.arg_infile,
                         err);
                exit(1);
            }
        };
        let (major, minor) = reader.version();
        println!("version: {}.{}", major, minor);
        match reader.header_information_as_str() {
            Ok(info) => println!("header information:\n{}", info),
            Err(err) => println!("WARNING: cannot display header information: {}", err),
        }
        if !args.flag_brief {
            let points: Vec<_> = reader.into_iter().collect();
            println!("number of points: {}", points.len());
        }
        exit(0);
    }
    unreachable!()
}
