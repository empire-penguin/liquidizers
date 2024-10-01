//! Module: liquid-dsp

#![allow(trivial_numeric_casts)]

use libc::{c_int, c_long, c_short};
use liquidizers_sys::__BindgenComplex;
use num_complex;

use std::ffi;
use std::fmt;
use std::marker::PhantomData;
use std::os::raw::c_void;

use std::result;
use std::string::FromUtf8Error;
use std::sync::Arc;
use std::{mem, ptr, str};

// `liquid`-specific Result type.
pub type Result<T> = result::Result<T, Error>;

// `liquid`-specific Complex type.
pub type Complex<T> = num_complex::Complex<T>;

/// An error returned by a liquid API function.
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Error {
    LiquidOk,
    LiquidEint,
    LiquidEiobj,
    LiquidEiconfig,
    LiquidEival,
    LiquidEirange,
    LiquidEimode,
    LiquidEumode,
    LiquidEnoinit,
    LiquidEimem,
    LiquidEio,
    LiquidEnoconv,
    LiquidEnoimp,
}

impl Error {
    pub fn to_raw(self) -> usize {
        match self {
            Error::LiquidOk => 0,
            Error::LiquidEint => 1,
            Error::LiquidEiobj => 2,
            Error::LiquidEiconfig => 3,
            Error::LiquidEival => 4,
            Error::LiquidEirange => 5,
            Error::LiquidEimode => 6,
            Error::LiquidEumode => 7,
            Error::LiquidEnoinit => 8,
            Error::LiquidEimem => 9,
            Error::LiquidEio => 10,
            Error::LiquidEnoconv => 11,
            Error::LiquidEnoimp => 12,
        }
    }

    pub fn from_raw(raw: i32) -> Error {
        assert!(raw >= 0, "Negative error code: {}", raw);
        match raw as u32 {
            liquidizers_sys::liquid_error_code_LIQUID_OK => Error::LiquidOk,
            liquidizers_sys::liquid_error_code_LIQUID_EINT => Error::LiquidEint,
            liquidizers_sys::liquid_error_code_LIQUID_EIOBJ => Error::LiquidEiobj,
            liquidizers_sys::liquid_error_code_LIQUID_EICONFIG => Error::LiquidEiconfig,
            liquidizers_sys::liquid_error_code_LIQUID_EIVAL => Error::LiquidEival,
            liquidizers_sys::liquid_error_code_LIQUID_EIRANGE => Error::LiquidEirange,
            liquidizers_sys::liquid_error_code_LIQUID_EIMODE => Error::LiquidEimode,
            liquidizers_sys::liquid_error_code_LIQUID_EUMODE => Error::LiquidEumode,
            liquidizers_sys::liquid_error_code_LIQUID_ENOINIT => Error::LiquidEnoinit,
            liquidizers_sys::liquid_error_code_LIQUID_EIMEM => Error::LiquidEimem,
            liquidizers_sys::liquid_error_code_LIQUID_EIO => Error::LiquidEio,
            liquidizers_sys::liquid_error_code_LIQUID_ENOCONV => Error::LiquidEnoconv,
            liquidizers_sys::liquid_error_code_LIQUID_ENOIMP => Error::LiquidEnoimp,
            x => unsafe {
                let s = liquidizers_sys::liquid_error_str[x as usize];
                panic!(
                    "Unknown error [{}]: {}",
                    x,
                    str::from_utf8(ffi::CStr::from_ptr(s).to_bytes()).unwrap()
                );
            },
        }
    }

    pub fn message(self) -> &'static str {
        unsafe {
            let s = liquidizers_sys::liquid_error_str[self.to_raw()];
            let v: &'static [u8] = mem::transmute(ffi::CStr::from_ptr(s).to_bytes());
            str::from_utf8(v).unwrap()
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        self.message()
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // FIXME: An unquoted string is not a good `Debug` output.
        write!(f, "{}", self.message())
    }
}

impl From<Error> for std::io::Error {
    fn from(error: Error) -> Self {
        use std::io::ErrorKind;

        let kind = match error {
            Error::LiquidOk => ErrorKind::Other,
            Error::LiquidEint => ErrorKind::InvalidInput,
            Error::LiquidEiobj => ErrorKind::InvalidData,
            Error::LiquidEiconfig => ErrorKind::InvalidInput,
            Error::LiquidEival => ErrorKind::InvalidInput,
            Error::LiquidEirange => ErrorKind::InvalidInput,
            Error::LiquidEimode => ErrorKind::InvalidInput,
            Error::LiquidEumode => ErrorKind::InvalidInput,
            Error::LiquidEnoinit => ErrorKind::Other,
            Error::LiquidEimem => ErrorKind::Other,
            Error::LiquidEio => ErrorKind::Other,
            Error::LiquidEnoconv => ErrorKind::Other,
            Error::LiquidEnoimp => ErrorKind::Other,

            _ => ErrorKind::Other,
        };

        std::io::Error::new(kind, error)
    }
}

pub mod agc;

pub fn version() -> String {
    unsafe {
        let cstr = ffi::CStr::from_ptr(liquidizers_sys::liquid_libversion());
        cstr.to_string_lossy().into_owned()
    }
}
