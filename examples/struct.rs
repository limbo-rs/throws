#[macro_use]
extern crate throws;

use std::io;
use std::num::{ParseIntError, ParseFloatError};

struct SomeStruct;

throws! {
    FooError = Io(io::Error), Parse(ParseIntError);
    BarError = ParseFloat(ParseFloatError), Foo(FooError)
}
impl SomeStruct {
    fn foo(&self) -> Result<(), FooError> {
        Err(From::from(io::Error::from(io::ErrorKind::NotFound)))
    }

    fn bar(&self) -> Result<(), BarError> {
        let foo = self.foo()?;
        Ok(foo)
    }
}

fn main() {
    let s = SomeStruct;
    match s.bar() {
        Err(ref err) => println!("ERROR: {}", err),
        Ok(_) => println!("OK"),
    }
}