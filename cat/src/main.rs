extern crate clap;

use clap::{Arg, App};

use std::fs::File;
use std::io::{self, Read};
use std::process;

fn main() {
    let matches = App::new("cat")
        .version("0.1.0")
        .author("nokaa <nokaa@cock.li>")
        .about("Concatenate files")
        .arg(Arg::with_name("FILE")
             .help("Sets input file to use")
             .required(false)
             .index(1))
        .get_matches();

    if let Some(input) = matches.value_of("FILE") {
        let file = match read_file(input) {
            Ok(f) => f,
            Err(e) => {
                println!("{}", e);
                process::exit(-1);
            }
        };

        let contents = String::from_utf8(file).unwrap();
        println!("{}", contents);
    } else {
        if let Err(e) = io::copy(&mut io::stdin(), &mut io::stdout()) {
            println!("{}", e);
            process::exit(-1);
        }
    }
}

/// Read file `filename` into a `Vec<u8>`.
fn read_file(filename: &str) -> Result<Vec<u8>, io::Error> {
    let mut f = try!(File::open(filename));
    let mut buf: Vec<u8> = vec![];
    try!(f.read_to_end(&mut buf));

    Ok(buf)
}
