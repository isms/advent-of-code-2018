use std::fs::File;
use std::io::prelude::*;

pub fn read_input(filename: &str) -> String {
    let mut f = File::open(filename).expect("File not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Problem reading the file");
    contents
}
