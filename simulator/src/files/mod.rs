pub mod charmap;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn charmap(path: &str) -> std::io::Result<Vec<u8>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    while let Ok(_) = reader.read_line(&mut line) {
        line.clear();
    }
    todo!()
}
