/*!
This crate provides macros for deriving some useful methods and traits for Error enums.

All of these macros are designed to be used with the
[`custom_derive`](https://crates.io/crates/custom_derive) crate, though they can be used independent of it.

# Example

Derive implementation of the standard `Error` trait.
This also provides the requisite `Display` implementation.

```rust
#[macro_use] extern crate custom_derive;
#[macro_use] extern crate error_derive;

# use std::error::Error as StdError;
# use std::io;
# use std::str::{from_utf8, Utf8Error};

custom_derive! {
    #[derive(Debug, ErrorFrom, Error("very bad error"))]
    pub enum Error {
        Io(io::Error),
        Utf8(Utf8Error),
    }
}

# fn main() {
let io_error = Error::Io(io::Error::last_os_error());
let utf8_error = Error::Utf8(from_utf8(&[0, 142]).unwrap_err());

assert_eq!("very bad error", io_error.description());
assert_eq!("very bad error", utf8_error.description());

assert!(io_error.cause().is_some());
assert!(utf8_error.cause().is_some());
# }
```

# Overview

This crate allows to derive the following traits:

- `ErrorFrom`, which creates `From` trait implementations from each enum variants that wraps an inner `Error`
- `Error`, which implements the standard `Error` trait with a given description

> **Note**: `Error` currently also derives `Display`, but this will most likely be decoupled in the future.

# Usage without `custom_derive!`

Although designed to be used with `custom_derive!`, all of the macros in this crate can be used without it.
The following:

```rust
# #[macro_use] extern crate custom_derive;
# #[macro_use] extern crate error_derive;
# use std::io;
custom_derive! {
    #[derive(Debug, Error("just I/O error"))]
    pub enum JustIoError { ThisOne(io::Error) }
}
# fn main() {}
```

Can also be writtten as:

```rust
# #[macro_use] extern crate custom_derive;
# #[macro_use] extern crate error_derive;
# use std::io;
#[derive(Debug)]
pub enum JustIoError { ThisOne(io::Error) }
Error! { ("just I/O error") pub enum JustIoError { ThisOne(io::Error) } }
# fn main() {}
```
*/

#![doc(html_root_uri = "https://xion.github.io/rust-error-derive")]

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
