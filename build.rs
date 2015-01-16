#![allow(unstable)]

extern crate gl_generator;
extern crate khronos_api;

use std::os;
use std::io::File;

#[cfg(not(target_os = "android"))]
/// OpenGL 3.0 bindings for Linux/Mac
fn main() {
    let dest = Path::new(os::getenv("OUT_DIR").unwrap());

    let mut file = File::create(&dest.join("gl_bindings.rs")).unwrap();
    gl_generator::generate_bindings(gl_generator::GlobalGenerator,
                                    gl_generator::registry::Ns::Gl,
                                    khronos_api::GL_XML,
                                    vec!["GL_ARB_texture_rectangle".to_string()],
                                    "3.0", "core", &mut file).unwrap();
}

#[cfg(target_os = "android")]
/// EGL 2.0 bindings for Android
fn main() {
    let dest = Path::new(os::getenv("OUT_DIR").unwrap());

    let mut file = File::create(&dest.join("gl_bindings.rs")).unwrap();

    gl_generator::generate_bindings(gl_generator::StaticGenerator,
                                    gl_generator::registry::Ns::Gles2,
                                    khronos_api::EGL_XML,
                                    vec!["GL_EXT_texture_format_BGRA8888".to_string()],
                                    "2.0", "core", &mut file).unwrap();
}