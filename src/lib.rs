//! Module: liquid-dsp

#![allow(trivial_numeric_casts)]

use libc::{c_int, c_long, c_short};

use std::ffi;
use std::marker::PhantomData;
use std::os::raw::c_void;

use std::result;
use std::string::FromUtf8Error;
use std::sync::Arc;
use std::{mem, ptr, str};

// `liquid`-specific Result type.
pub type Result<T> = result::Result<T, Error>;

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
    pub fn to_raw(self) -> i32 {
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
        match raw {
            0 => Error::LiquidOk,
            1 => Error::LiquidEint,
            2 => Error::LiquidEiobj,
            3 => Error::LiquidEiconfig,
            4 => Error::LiquidEival,
            5 => Error::LiquidEirange,
            6 => Error::LiquidEimode,
            7 => Error::LiquidEumode,
            8 => Error::LiquidEnoinit,
            9 => Error::LiquidEimem,
            10 => Error::LiquidEio,
            11 => Error::LiquidEnoconv,
            12 => Error::LiquidEnoimp,
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
            let i = self.to_raw();
            let s = liquidizers_sys::liquid_error_str[i as usize];
            let v: &'static [u8] = mem::transmute(ffi::CStr::from_ptr(s).to_bytes());
            str::from_utf8(v).unwrap()
        }
    }
}

pub fn version() -> String {
    unsafe {
        let cstr = ffi::CStr::from_ptr(liquidizers_sys::liquid_libversion());
        cstr.to_string_lossy().into_owned()
    }
}
