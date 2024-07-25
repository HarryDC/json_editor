mod parse_all;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

fn read_file<P : AsRef<Path>>(path: P) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    println!("Hello, world!");
}
