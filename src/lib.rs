// Copyright 2014 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![crate_name = "gleam"]
#![comment = "Servo OpenGL bindings"]
#![license = "ASL2"]
#![crate_type = "lib"]

#![feature(phase)]
#![feature(globs)]

#[phase(plugin)]
extern crate gl_generator;

extern crate libc;

pub mod gl;

mod ffi {
    /// OpenGL 3.0 bindings for Linux/Mac
    #[cfg(not(target_os = "android"))]
	generate_gl_bindings!{
		api: "gl",
		profile: "core",
		version: "3.0",
		generator: "global",
		extensions: [ "GL_ARB_texture_rectangle" ]
	}

    // EGL 2.0 bindings for Android
    #[cfg(target_os = "android")]
	generate_gl_bindings!{
		api: "gles2",
		profile: "core",
		version: "2.0",
		generator: "static",
		extensions: [ "GL_EXT_texture_format_BGRA8888" ]
	}
}
