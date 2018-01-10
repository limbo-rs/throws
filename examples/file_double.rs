#[macro_use]
extern crate throws;

use std::fs::File;
use std::io::{self, Read};
use std::num::ParseIntError;

throws!(FileDoubleError = Io(io::Error), Parse(ParseIntError));
fn file_double(path: &str) -> Result<i32, FileDoubleError> {
    let mut file = File::open(path)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;
    let n: i32 = contents.trim().parse()?;

    Ok(2 * n)
}

fn main() {
    let result = file_double("test");
    match result {
        Err(ref err) => println!("ERROR: {}", err),
        Ok(n) => println!("OK: {}", n),
    }
    println!("RESULT: {:?}", result);
}