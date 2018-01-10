#![allow(unused_imports)]

#[macro_export]
macro_rules! throws {
    ($($(#[$attr:meta])* $name:ident = $($case:ident($error:path)),*);+) => {
        throws!($($(#[$attr])* () $name = $($case($error)),*);+);
    };
    ($($(#[$attr:meta])* pub $name:ident = $($case:ident($error:path)),*);+) => {
        throws!($($(#[$attr])* (pub) $name = $($case($error)),*);+);
    };
    ($($(#[$attr:meta])* pub($($restriction:tt)+) $name:ident = $($case:ident($error:path)),*);+) => {
        throws!($($(#[$attr])* (pub($($restriction)+)) $name = $($case($error)),*);+);
    };
    ($($(#[$attr:meta])* ($($vis:tt)*) $name:ident = $($case:ident($error:path)),*);+) => {
    $(
        $(#[$attr])*
        #[derive(Debug)]
        $($vis)* enum $name { $($case($error)),* }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                    $($name::$case(ref err) => write!(formatter, stringify!($case error: {}), err)),*
                }
            }
        }

        impl ::std::error::Error for $name {
            fn description(&self) -> &str {
                match *self {
                    $($name::$case(ref err) => err.description()),*
                }
            }

            fn cause(&self) -> Option<&::std::error::Error> {
                match *self {
                    $($name::$case(ref err) => Some(err)),*
                }
            }
        }

        $(
        impl ::std::convert::From<$error> for $name {
            fn from(err: $error) -> $name {
                $name::$case(err)
            }
        }
        )*
    )+
    };
}

#[cfg(test)]
mod tests {
    use std::io;
    use std::num::{ParseIntError, ParseFloatError};

    throws!(SingleIoError = Io(io::Error));

    throws!(#[derive(PartialEq, Eq)] SingleParseIntError = Parse(ParseIntError));

    throws!(MultipleError = Io(io::Error), ParseFloat(ParseFloatError), Single(SingleIoError));

    throws! {
        IoError = Io(io::Error);
        ParseError = Parse(ParseIntError)
    }

    mod error {
        use super::*;

        throws!(pub PubError = Io(io::Error), Parse(ParseIntError));

        throws!(pub(crate) PubCrateError = Io(io::Error), Parse(ParseIntError));

        throws!(
            #[derive(PartialEq, Eq, Clone)]
            #[allow(missing_docs)]
            pub(crate) AllFeaturesError = ParseInt(ParseIntError), ParseFloat(ParseFloatError)
        );

        throws! {
            () PrivateIoError = Io(io::Error);
            #[derive(Clone)]
            (pub) PubParseError = Parse(ParseIntError)
        }
    }

    #[test]
    fn create_from_error() {
        let error = From::from(io::Error::from(io::ErrorKind::NotFound));
        assert!(match error {
            SingleIoError::Io(_) => true,
        });

        let error = "not int"
            .parse::<i32>()
            .map_err(SingleParseIntError::from)
            .unwrap_err();
        assert!(match error {
            SingleParseIntError::Parse(_) => true,
        });
        assert_eq!(error, error);

        let error = From::from(io::Error::from(io::ErrorKind::NotFound));
        assert!(match error {
            MultipleError::Io(_) => true,
            _ => false,
        });

        let error = SingleIoError::from(io::Error::from(io::ErrorKind::NotFound));
        let error = From::from(error);
        assert!(match error {
            MultipleError::Single(_) => true,
            _ => false,
        });

        let error = From::from("not float".parse::<f32>().unwrap_err());
        assert!(match error {
            MultipleError::ParseFloat(_) => true,
            _ => false,
        });

        let error = From::from(io::Error::from(io::ErrorKind::NotFound));
        assert!(match error {
            error::PubError::Io(_) => true,
            _ => false,
        });

        let error = From::from("not int".parse::<i32>().unwrap_err());
        assert!(match error {
            error::PubCrateError::Parse(_) => true,
            _ => false,
        });

        let error = From::from("not float".parse::<f32>().unwrap_err());
        assert!(match error {
            error::AllFeaturesError::ParseFloat(_) => true,
            _ => false,
        });
        assert_eq!(error, error.clone());

        let error = From::from(io::Error::from(io::ErrorKind::NotFound));
        assert!(match error {
            IoError::Io(_) => true,
        });
    }
}
