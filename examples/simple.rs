#[macro_use]
extern crate throws;

use std::num::ParseIntError;

throws!(SimpleError = Parse(ParseIntError));
fn foo(source: &str) -> Result<i32, SimpleError> {
    let n: i32 = source.trim().parse()?;
    Ok(n)
}

fn main() {
    let result = foo("test");
    match result {
        Err(ref err) => println!("ERROR: {}", err),
        Ok(n) => println!("OK: {}", n),
    }
    println!("RESULT: {:?}", result);
}