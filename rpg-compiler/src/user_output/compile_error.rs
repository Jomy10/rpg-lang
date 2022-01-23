use std::io;
use std::num::ParseIntError;
#[macro_export]
/// Prints out an compilation error message in red.
macro_rules! compile_error {
    ($( $arg: tt)*) => ({
        let s = format!($($arg)*);
        eprintln!("{}", simple_colors::red!(s));
        std::process::exit(1)
    })
}

pub static mut VERBOSE: bool = false;

pub trait CompileError<T> {
    fn expect_compile_error(self, msg: &str) -> T;
}

impl<T> CompileError<T> for Result<T, ParseIntError> {
    fn expect_compile_error(self, msg: &str) -> T {
        match self {
            Ok(t) => t,
            Err(e) => unsafe {
                if VERBOSE {
                    crate::compile_error!("{}\n== VERBOSE OUTPUT ==\n{}", msg, e)
                } else {
                    crate::compile_error!("{}", msg)
                }
            }
        }
    }
}

impl<T> CompileError<T> for Result<T, String> {
    fn expect_compile_error(self, msg: &str) -> T {
        match self {
            Ok(t) => t,
            Err(e) => unsafe {
                if VERBOSE {
                    crate::compile_error!("{}\n== VERBOSE OUTPUT ==\n{}", msg, e)
                } else {
                    crate::compile_error!("{}", msg)
                }
            }
        }
    }
}

impl<T> CompileError<T> for Option<T> {
    fn expect_compile_error(self, msg: &str) -> T {
        match self {
            Some(t) => t,
            None => crate::compile_error!("{}", msg)
        }
    }
}

impl<T> CompileError<T> for io::Result<T> {
    fn expect_compile_error(self, msg: &str) -> T {
        match self {
            Ok(t) => t,
            Err(e) => unsafe {
                if VERBOSE {
                    crate::compile_error!("{}\n== VERBOSE OUTPUT ==\n{}", msg, e)
                } else {
                    crate::compile_error!("{}", msg)
                }
            }
        }
    }
}