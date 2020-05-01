extern crate gl;

use gl::*;
use std::fs::File;
use std::io::prelude::*;

pub struct Shader {
    shader_program: u32,
    shaders: [u32; 2],
    uniforms: [u32; 3],
}

pub fn create_shader(file_name: &str, shader_type: u32) -> u32 {
    unsafe {
        let shader = gl::CreateShader(shader_type);
        if shader == 0 {
            println!("Shader creation failed");
        }

        let mut shader_source: u8;
        let mut shader_source_len: i32;
    }
    0
}

pub fn load_shader(shader_file: &str) -> String {
    println!("Load shader: {}", shader_file);
    let mut file = File::open(shader_file).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    content
}

pub fn check_shader_error(shader: u32, flag: u32, isProgram: bool, msg: &str) {
    unsafe {
        let mut success = 0;
        let mut error: [i8; 512] = [0; 512];
        if isProgram {
            gl::GetProgramiv(shader, flag, &mut success);
        } else {
            gl::GetShaderiv(shader, flag, &mut success);
        }
        if success as u8 == gl::FALSE {
            if isProgram {
                gl::GetProgramInfoLog(shader, 512, 0 as *mut i32, error.as_mut_ptr());
            } else {
                gl::GetShaderInfoLog(shader, 512, 0 as *mut i32, error.as_mut_ptr());
            }
        }
    }
}

pub fn bind_shader(shader: &Shader) {
    unsafe {
        gl::UseProgram(shader.shader_program);
    }
}
