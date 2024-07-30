use json_parser::json;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use json::to_object;

fn read_file<P : AsRef<Path>>(path: P) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {

    let val = read_file("D:/experiments/rust/json_parser/tests/64KB.json").expect("Failed TO Load");
    let json = to_object(&*val);
    print!("Done");
}
