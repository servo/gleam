// Copyright 2014 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use libc::{c_int, c_void};
use std::mem;
use std::mem::size_of;
use std::ptr;
use std::str::{self, from_utf8};
use std::{string, slice};
use std::iter::repeat;
use std::ffi::{CString, c_str_to_bytes_with_nul};
use ffi;

pub use ffi::types::*;
pub use ffi::*;

#[inline]
pub fn buffer_data<T>(target: GLenum, data: &[T], usage: GLenum) {
    unsafe {
        ffi::BufferData(target,
                       (data.len() * size_of::<T>()) as GLsizeiptr,
                       data.as_ptr() as *const GLvoid,
                       usage);
    }
}

pub fn shader_source(shader: GLuint, strings: &[&[u8]]) {
    let pointers: Vec<*const u8> = strings.iter().map(|string| (*string).as_ptr()).collect();
    let lengths: Vec<GLint> = strings.iter().map(|string| string.len() as GLint).collect();
    unsafe {
        ffi::ShaderSource(shader, pointers.len() as GLsizei,
                         pointers.as_ptr() as *const *const GLchar, lengths.as_ptr());
    }
    drop(lengths);
    drop(pointers);
}

#[inline]
pub fn delete_buffers(buffers: &[GLuint]) {
    unsafe {
        ffi::DeleteBuffers(buffers.len() as GLsizei, buffers.as_ptr());
    }
}

#[inline]
pub fn delete_frame_buffers(frame_buffers: &[GLuint]) {
    unsafe {
        ffi::DeleteFramebuffers(frame_buffers.len() as GLsizei, frame_buffers.as_ptr());
    }
}

pub fn read_pixels(x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: GLenum, pixel_type: GLenum) -> Vec<u8> {
    let colors = match format {
        ffi::RGB => 3,
        ffi::RGBA => 3,
        _ => panic!("unsupported format for read_pixels"),
    };
    let depth = match pixel_type {
        ffi::UNSIGNED_BYTE => 1,
        _ => panic!("unsupported pixel_type for read_pixels"),
    };

    let len = (width * height * colors * depth) as uint;
    let mut pixels: Vec<u8> = Vec::new();
    pixels.reserve(len);

    unsafe {
        // We don't want any alignment padding on pixel rows.
        ffi::PixelStorei(ffi::PACK_ALIGNMENT, 1);
        ffi::ReadPixels(x, y, width, height, format, pixel_type, pixels.as_mut_ptr() as *mut c_void);
        pixels.set_len(len);
    }

    pixels
}

#[inline]
pub fn gen_buffers(n: GLsizei) -> Vec<GLuint> {
    unsafe {
        let mut result: Vec<_> = repeat(0 as GLuint).take(n as uint).collect();
        ffi::GenBuffers(n, result.as_mut_ptr());
        return result;
    }
}

#[inline]
pub fn gen_framebuffers(n: GLsizei) -> Vec<GLuint> {
    unsafe {
        let mut result: Vec<_> = repeat(0 as GLuint).take(n as uint).collect();
        ffi::GenFramebuffers(n, result.as_mut_ptr());
        return result;
    }
}

#[inline]
pub fn gen_textures(n: GLsizei) -> Vec<GLuint> {
    unsafe {
        let mut result: Vec<_> = repeat(0 as GLuint).take(n as uint).collect();
        ffi::GenTextures(n, result.as_mut_ptr());
        return result;
    }
}

#[inline]
pub fn active_texture(texture: GLenum) {
    unsafe {
        ffi::ActiveTexture(texture);
    }
}

#[inline]
pub fn attach_shader(program: GLuint, shader: GLuint) {
    unsafe {
        ffi::AttachShader(program, shader);
    }
}

#[inline]
pub fn bind_attrib_location(program: GLuint, index: GLuint, name: &str) {
    unsafe {
        ffi::BindAttribLocation(program, index, CString::from_slice(name.as_bytes()).as_ptr());
    }
}

#[inline]
pub fn bind_buffer(target: GLenum, buffer: GLuint) {
    unsafe {
        ffi::BindBuffer(target, buffer);
    }
}

#[inline]
pub fn bind_framebuffer(target: GLenum, framebuffer: GLuint) {
    unsafe {
        ffi::BindFramebuffer(target, framebuffer);
    }
}

#[inline]
pub fn bind_texture(target: GLenum, texture: GLuint) {
    unsafe {
        ffi::BindTexture(target, texture);
    }
}

// FIXME: Does not verify buffer size -- unsafe!
pub fn tex_image_2d(target: GLenum,
                    level: GLint,
                    internal_format: GLint,
                    width: GLsizei,
                    height: GLsizei,
                    border: GLint,
                    format: GLenum,
                    ty: GLenum,
                    opt_data: Option<&[u8]>) {
    match opt_data {
        Some(data) => {
            unsafe {
                let pdata = mem::transmute(data.as_ptr());
                ffi::TexImage2D(target, level, internal_format, width, height, border, format, ty,
                               pdata);
            }
        }
        None => {
            unsafe {
                ffi::TexImage2D(target, level, internal_format, width, height, border, format, ty,
                               ptr::null());
            }
        }
    }
}

#[inline]
pub fn tex_parameter_i(target: GLenum, pname: GLenum, param: GLint) {
    unsafe {
        ffi::TexParameteri(target, pname, param);
    }
}

#[inline]
pub fn framebuffer_texture_2d(target: GLenum,
                              attachment: GLenum,
                              textarget: GLenum,
                              texture: GLuint,
                              level: GLint) {
    unsafe {
        ffi::FramebufferTexture2D(target, attachment, textarget, texture, level);
    }
}

#[inline]
pub fn vertex_attrib_pointer_f32(index: GLuint,
                                 size: GLint,
                                 normalized: bool,
                                 stride: GLsizei,
                                 offset: GLuint) {
    unsafe {
        ffi::VertexAttribPointer(index,
                                size,
                                ffi::FLOAT,
                                normalized as GLboolean,
                                stride,
                                mem::transmute(offset as uint));
    }
}

#[inline]
pub fn delete_textures(textures: &[GLuint]) {
    unsafe {
        ffi::DeleteTextures(textures.len() as GLsizei, textures.as_ptr());
    }
}

#[inline]
pub fn viewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
    unsafe {
        ffi::Viewport(x, y, width, height);
    }
}

#[inline]
pub fn line_width(width: GLfloat) {
    unsafe {
        ffi::LineWidth(width);
    }
}

#[inline]
pub fn use_program(program: GLuint) {
    unsafe {
        ffi::UseProgram(program);
    }
}

#[inline]
pub fn draw_arrays(mode: GLenum, first: GLint, count: GLsizei) {
    unsafe {
        return ffi::DrawArrays(mode, first, count);
    }
}

#[inline]
pub fn blend_func(sfactor: GLenum, dfactor: GLenum) {
    unsafe {
        ffi::BlendFunc(sfactor, dfactor);
    }
}

#[inline]
pub fn enable(cap: GLenum) {
    unsafe {
        ffi::Enable(cap);
    }
}

#[inline]
pub fn disable(cap: GLenum) {
    unsafe {
        ffi::Disable(cap);
    }
}

#[inline]
pub fn enable_vertex_attrib_array(index: GLuint) {
    unsafe {
        ffi::EnableVertexAttribArray(index);
    }
}

#[inline]
pub fn disable_vertex_attrib_array(index: GLuint) {
    unsafe {
        ffi::DisableVertexAttribArray(index);
    }
}

#[inline]
pub fn uniform_4f(location: GLint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat) {
    unsafe {
        ffi::Uniform4f(location, x, y, z, w);
    }
}

#[inline]
pub fn uniform_matrix_4fv(location: GLint, transpose: bool, value: &[f32]) {
    unsafe {
        ffi::UniformMatrix4fv(location,
                               1 as GLsizei,
                               transpose as GLboolean,
                               mem::transmute(&value[0]));
    }
}

#[inline]
pub fn uniform_1i(location: GLint, x: GLint) {
    unsafe {
        ffi::Uniform1i(location, x);
    }
}

#[inline]
pub fn get_attrib_location(program: GLuint, name: &str) -> c_int {
    unsafe {
        ffi::GetAttribLocation(program, CString::from_slice(name.as_bytes()).as_ptr() as *const GLchar)
    }
}

#[inline]
pub fn get_uniform_location(program: GLuint, name: &str) -> c_int {
    unsafe {
        ffi::GetUniformLocation(program, CString::from_slice(name.as_bytes()).as_ptr() as *const GLchar)
    }
}

pub fn get_program_info_log(program: GLuint) -> String {
    unsafe {
        let mut result: Vec<_> = repeat(0u8).take(1024u).collect();
        let mut result_len: GLsizei = 0 as GLsizei;
        ffi::GetProgramInfoLog(program,
                            1024 as GLsizei,
                            &mut result_len,
                            result.as_ptr() as *mut GLchar);
        result.truncate(if result_len > 0 {result_len as uint - 1u} else {0u});
        from_utf8(result.as_slice()).unwrap().to_string()
    }
}

#[inline]
pub fn get_program_iv(program: GLuint, pname: GLenum) -> GLint {
    unsafe {
        let mut result: GLint = 0 as GLint;
        ffi::GetProgramiv(program, pname, &mut result);
        return result;
    }
}

pub fn get_shader_info_log(shader: GLuint) -> String {
    unsafe {
        let mut result: Vec<_> = repeat(0u8).take(1024u).collect();
        let mut result_len: GLsizei = 0 as GLsizei;
        ffi::GetShaderInfoLog(shader,
                           1024 as GLsizei,
                           &mut result_len,
                           result.as_ptr() as *mut GLchar);
        result.truncate(if result_len > 0 {result_len as uint - 1u} else {0u});
        from_utf8(result.as_slice()).unwrap().to_string()
    }
}

#[inline]
pub fn get_string(which: GLenum) -> String {
    unsafe {
        let llstr = ffi::GetString(which);
        if !llstr.is_null() {
            return str::from_utf8_unchecked(c_str_to_bytes_with_nul(&(llstr as *const i8))
                                             ).to_string();
        } else {
            return "".to_string();
        }
    }
}

#[inline]
pub fn get_shader_iv(shader: GLuint, pname: GLenum) -> GLint {
    unsafe {
        let mut result: GLint = 0 as GLint;
        ffi::GetShaderiv(shader, pname, &mut result);
        return result;
    }
}

#[inline]
pub fn compile_shader(shader: GLuint) {
    unsafe {
        ffi::CompileShader(shader);
    }
}

#[inline]
pub fn create_program() -> GLuint {
    unsafe {
        return ffi::CreateProgram();
    }
}

#[inline]
pub fn create_shader(shader_type: GLenum) -> GLuint {
    unsafe {
        return ffi::CreateShader(shader_type);
    }
}

#[inline]
pub fn link_program(program: GLuint) {
    unsafe {
        return ffi::LinkProgram(program);
    }
}

#[inline]
pub fn clear_color(r: f32, g: f32, b: f32, a: f32) {
    unsafe {
        ffi::ClearColor(r, g, b, a);
    }
}

#[inline]
pub fn clear(buffer_mask: GLbitfield) {
    unsafe {
        ffi::Clear(buffer_mask);
    }
}

#[inline]
pub fn flush() {
    unsafe {
        ffi::Flush();
    }
}

#[inline]
pub fn finish() {
    unsafe {
        ffi::Finish();
    }
}

#[cfg(target_os="android")]
extern {
    pub fn glEGLImageTargetTexture2DOES(target: GLenum, image: GLeglImageOES);
}

#[cfg(target_os="android")]
pub fn egl_image_target_texture2d_oes(target: GLenum, image: GLeglImageOES) {
    unsafe {
        glEGLImageTargetTexture2DOES(target, image);
    }
}
