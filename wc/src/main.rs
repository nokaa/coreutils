extern crate clap;

use clap::{Arg, App};

use std::fs::File;
use std::io::{self, Read};

fn main() {
    let matches = App::new("wc")
        .version("0.1.0")
        .author("nokaa <nokaa@cock.li>")
        .about("Print newline, word, and byte counts for each FILE, and a total line if\
        more than one FILE is specified. A word is a non-zero-length sequence of\
        characters delimited by white space.")
        .arg(Arg::with_name("FILE")
             .help("Sets the input file to use")
             .required(false)
             .index(1))
        .get_matches();

    if let Some(input) = matches.value_of("FILE") {
        let file = match read_file(input) {
            Ok(f) => f,
            Err(e) => panic!(format!("{}", e)),
        };

        let (lines, words, chars) = count(&file[..]);
        println!("{} {} {} {}", lines, words, chars, input);
    } else {
        let mut buf = String::new();
        let mut stdin = io::stdin();

        if let Err(e) = stdin.read_to_string(&mut buf) {
            panic!(format!("{}", e));
        }

        let (lines, words, chars) = count(buf.as_bytes());
        println!("{} {} {}", lines, words, chars);
    }
}

/// Read file `filename into a `Vec<u8>`.
fn read_file(filename: &str) -> Result<Vec<u8>, io::Error> {
    let mut f = try!(File::open(filename));
    let mut buf: Vec<u8> = vec![];
    try!(f.read_to_end(&mut buf));

    Ok(buf)
}

fn count(input: &[u8]) -> (usize, usize, usize) {
    let mut lines = 0usize;
    let mut words = 0usize;
    let mut chars = 0usize;
    let mut in_word = false;

    let mut data = input.into_iter();

    while let Some(&c) = data.next() {
        if in_word && is_whitespace(c) {
            words += 1;
            in_word = false;
        } else if !in_word && !is_whitespace(c) {
            in_word = true;
        }

        if b'\n' == c {
            lines += 1;
        }

        chars += 1;
    }

    (lines, words, chars)
}

fn is_whitespace(c: u8) -> bool {
    match c {
        b' ' => true,
        b'\t' => true,
        b'\n' => true,
        b'\r' => true,
        _ => false,
    }
}
