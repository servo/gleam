// Copyright 2014 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![cfg_attr(not(feature = "std"), no_std)]
#![crate_name = "gleam"]
#![crate_type = "lib"]

#[macro_use]
extern crate alloc;

pub mod gl;

mod ffi {
    include!(concat!(env!("OUT_DIR"), "/gl_and_gles_bindings.rs"));
}

mod ffi_gl {
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}

mod ffi_gles {
    include!(concat!(env!("OUT_DIR"), "/gles_bindings.rs"));
}

pub(crate) mod utils {
    use alloc::vec::Vec;

    /// CString is not available on libcore, but all that CString does
    /// is to cast the string to bytes, then append a "0" at the end.
    pub fn cstring_from_str(s: &str) -> Vec<u8> { let mut v: Vec<u8> = s.into(); v.push(0); v }
}