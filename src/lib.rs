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

use liquidizers_sys::*;

pub fn version() -> String {
    unsafe {
        let cstr = ffi::CStr::from_ptr(liquidizers_sys::liquid_libversion());
        cstr.to_string_lossy().into_owned()
    }
}

// `liquid`-specific Result type.
// pub type Result<T> = result::Result<T, Error>;
