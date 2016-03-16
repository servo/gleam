extern crate gl_generator;
extern crate khronos_api;

use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

fn main() {
    let dest = env::var("OUT_DIR").unwrap();

    let mut file = BufWriter::new(File::create(&Path::new(&dest).join("gl_bindings.rs")).unwrap());

    let target = env::var("TARGET").unwrap();
    if target.contains("android") {
        // GLES 2.0 bindings for Android
        gl_generator::generate_bindings(gl_generator::StaticGenerator,
                                        gl_generator::registry::Ns::Gles2,
                                        gl_generator::Fallbacks::All,
                                        khronos_api::GL_XML,
                                        vec!["GL_EXT_texture_format_BGRA8888".to_string()],
                                        "3.0", "core", &mut file).unwrap();
        println!("cargo:rustc-link-lib=GLESv2");
    } else {
        // OpenGL 3.3 bindings for Linux/Mac/Windows
        gl_generator::generate_bindings(gl_generator::GlobalGenerator,
                                        gl_generator::registry::Ns::Gl,
                                        gl_generator::Fallbacks::All,
                                        khronos_api::GL_XML,
                                        vec!["GL_ARB_texture_rectangle".to_string()],
                                        "3.3", "core", &mut file).unwrap();
        if target.contains("linux") {
            println!("cargo:rustc-link-lib=GL");
        } else if target.contains("windows") {
            println!("cargo:rustc-link-lib=opengl32");
        } else {
            println!("cargo:rustc-link-lib=framework=OpenGL");
        }

    }
}
