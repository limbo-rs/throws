#[macro_use]
extern crate throws;

mod my {
    use std::num::{ParseIntError, ParseFloatError};

    pub struct SomeStruct;

    throws! {
        #[derive(Clone)]
        pub FooError = ParseInt(ParseIntError);

        #[derive(Clone)]
        pub BarError = ParseFloat(ParseFloatError), Foo(FooError)
    }
    impl SomeStruct {
        fn foo(&self) -> Result<(), FooError> {
            Err(From::from("not int".trim().parse::<i32>().unwrap_err()))
        }

        pub fn bar(&self) -> Result<(), BarError> {
            let foo = self.foo()?;
            Ok(foo)
        }
    }
}

fn main() {
    let s = my::SomeStruct;
    match s.bar() {
        Ok(_) => println!("OK"),
        Err(my::BarError::Foo(ref err)) => println!("Foo ERROR: {}", err),
        Err(err) => println!("ERROR: {}", err),
    }
}