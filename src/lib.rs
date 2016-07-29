/*!
This crate provides macros for deriving some useful methods and traits for Error structs.

# Example

```rust
#[macro_use] extern crate custom_derive;
#[macro_use] extern crate error_derive;

# use std::io;
# use std::str::Utf8Error;

custom_derive! {
    #[derive(Debug, ErrorFrom, Error("very bad error"))]
    pub enum Error {
        Io(io::Error),
        Utf8(Utf8Error),
    }
}

# fn main() {

# }
```
*/

#[macro_use] mod util;


#[macro_export]
macro_rules! ErrorFrom {
    (@expand $name:ident ($($var_names:ident($var_tys:ty),)*)) => {
        $(
            impl ::std::convert::From<$var_tys> for $name {
                fn from(v: $var_tys) -> $name {
                    $name::$var_names(v)
                }
            }
        )*
    };

    (() $(pub)* enum $name:ident { $($body:tt)* }) => {
        enum_derive_util! {
            @collect_unary_variants
            (ErrorFrom { @expand $name }),
            ($($body)*,) -> ()
        }
    };
}


#[macro_export]
macro_rules! Error {
    (@expand $desc:expr, $name:ident ($($var_names:ident($var_tys:ty),)*)) => {
        impl ::std::fmt::Display for $name {
            fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                    $(
                        $name::$var_names(ref c) => write!(
                            fmt, concat!($desc, " ({})"), c),
                    )+
                }
            }
        }

        impl ::std::error::Error for $name {
            fn description(&self) -> &str {
               $desc
            }

            fn cause(&self) -> Option<&::std::error::Error> {
                match *self {
                    $(
                        $name::$var_names(ref c) => Some(c),
                    )*
                }
            }
        }
    };

    (($desc:expr) $(pub)* enum $name:ident { $($body:tt)* }) => {
        enum_derive_util! {
            @collect_unary_variants
            (Error { @expand $desc, $name }),
            ($($body)*,) -> ()
        }
    };
}
