extern crate url;
extern crate regex;
extern crate clap;
#[macro_use]
extern crate lazy_static;

use std::io::Read;
use std::io;
use std::fs;

use clap::{Arg, App, ArgMatches};
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(https?://[[:alpha:][:digit:][+#/\&\?;=%_\-\.{},]]+)").unwrap();
}

fn parse_options() -> ArgMatches<'static> {
    App::new("link-extract")
        .version("0.1.0")
        .author("Stone Tickle")
        .about("extract links")
        .arg(Arg::with_name("file")
             .help("file to operate on, if not present, operate on stdin"))
        .get_matches()
}

fn get_stdin() -> String {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buf = String::new();
    handle.read_to_string(&mut buf).unwrap();

    buf
}

fn get_file(filename: &str) -> String {
    let mut file = fs::File::open(filename).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    buf
}

fn main() {
    let options = parse_options();
    let string =
        match options.value_of("file") {
            Some(filename) => get_file(&filename),
            None => get_stdin()
        };

    for cap in RE.captures_iter(&string) {
        println!("{}", &cap[0]);
    }
}
