extern crate clap;

use clap::App;

use std::fs::File;
use std::io;

fn main() {
    let matches = App::new("touch")
        .version("0.1")
        .author("nokaa <nokaa@cock.li>")
        .about("Create an empty file")
        .arg_from_usage("<FILE> 'The file to create'")
        .get_matches();

    let filename = matches.value_of("FILE").unwrap();
    assert_eq!((), create_file(filename).unwrap());
}

fn file_exists(filename: &str) -> bool {
    match File::open(filename) {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn create_file(filename: &str) -> Result<(), io::Error> {
    if !file_exists(filename) {
        let _ = try!(File::create(filename));
    }

    Ok(())
}
