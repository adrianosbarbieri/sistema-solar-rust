extern crate gl;

use super::camera::Camera;
use super::transform::Transform;
use cgmath::*;
use std::ffi::CStr;
use std::ffi::CString;
use std::fs::File;
use std::io::prelude::*;

pub struct Shader {
    shader_program: u32,
    shaders: [u32; 2],
    uniforms: [i32; 3],
}

pub fn create_shader(text: &mut str, shader_type: u32) -> u32 {
    unsafe {
        let shader = gl::CreateShader(shader_type);
        let shader_cstring = CString::new(text.as_bytes()).expect("Error creating shader cstring");
        let shader_text = CStr::from_bytes_with_nul(shader_cstring.to_bytes_with_nul())
            .expect("Error creating shader cstr");
        if shader == 0 {
            println!("Shader creation failed");
        }

        let shader_text_array = [shader_text];

        gl::ShaderSource(
            shader,
            1,
            shader_text_array.as_ptr() as *const *const i8,
            std::ptr::null(),
        );
        gl::CompileShader(shader);

        check_shader_error(
            shader,
            gl::COMPILE_STATUS,
            false,
            "Shader compilation failed",
        );
        shader
    }
}

pub fn load_shader(shader_file: &str) -> String {
    println!("Load shader: {}", shader_file);
    let mut file = File::open(shader_file).unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    content
}

pub fn check_shader_error(shader: u32, flag: u32, is_program: bool, msg: &str) {
    unsafe {
        let mut success = 0;
        let mut error: [i8; 512] = [0; 512];
        if is_program {
            gl::GetProgramiv(shader, flag, &mut success);
        } else {
            gl::GetShaderiv(shader, flag, &mut success);
        }
        if success as u8 == gl::FALSE {
            if is_program {
                gl::GetProgramInfoLog(shader, 512, 0 as *mut i32, error.as_mut_ptr());
                panic!("{}", msg);
            } else {
                gl::GetShaderInfoLog(shader, 512, 0 as *mut i32, error.as_mut_ptr());
                panic!("{}", msg);
            }
        }
    }
}

impl Shader {
    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.shader_program);
        }
    }
    pub fn destroy(self) {
        unsafe {
            for i in 0..2 {
                gl::DetachShader(self.shader_program, self.shaders[i]);
                gl::DeleteShader(self.shaders[i]);
            }
            gl::DeleteProgram(self.shader_program);
        }
    }
    pub fn new(shader_file: &str) -> Shader {
        let vertex_shader_name = shader_file.to_owned() + ".vertex";
        let fragment_shader_name = shader_file.to_owned() + ".fragment";
        unsafe {
            let mut vertex_text = load_shader(vertex_shader_name.as_str());
            let mut frag_text = load_shader(fragment_shader_name.as_str());
            let shader1 = create_shader(vertex_text.as_mut_str(), gl::VERTEX_SHADER);
            let shader2 = create_shader(frag_text.as_mut_str(), gl::FRAGMENT_SHADER);

            let mut shader = Shader {
                shader_program: gl::CreateProgram(),
                shaders: [shader1, shader2],
                uniforms: [0, 0, 0],
            };

            gl::AttachShader(shader.shader_program, shader.shaders[0]);
            gl::AttachShader(shader.shader_program, shader.shaders[1]);
            gl::BindAttribLocation(
                shader.shader_program,
                0,
                "position\0".as_bytes().as_ptr() as *const i8,
            );
            gl::BindAttribLocation(
                shader.shader_program,
                1,
                "texture\0".as_bytes().as_ptr() as *const i8,
            );
            gl::BindAttribLocation(
                shader.shader_program,
                2,
                "normal\0".as_bytes().as_ptr() as *const i8,
            );
            gl::LinkProgram(shader.shader_program);
            check_shader_error(
                shader.shader_program,
                gl::LINK_STATUS,
                true,
                "shader linking failed",
            );
            gl::ValidateProgram(shader.shader_program);
            check_shader_error(
                shader.shader_program,
                gl::VALIDATE_STATUS,
                true,
                "shader invalid",
            );
            shader.uniforms[0] = gl::GetUniformLocation(
                shader.shader_program,
                "transform\0".as_bytes().as_ptr() as *const i8,
            );
            shader.uniforms[1] = gl::GetUniformLocation(
                shader.shader_program,
                "Normal\0".as_bytes().as_ptr() as *const i8,
            );
            shader.uniforms[2] = gl::GetUniformLocation(
                shader.shader_program,
                "camera\0".as_bytes().as_ptr() as *const i8,
            );
            shader
        }
    }
    pub fn update(&self, trans: &Transform, cam: &Camera) {
        unsafe {
            gl::UniformMatrix4fv(self.uniforms[0], 1, gl::FALSE, trans.get_model().as_ptr());
            gl::UniformMatrix4fv(
                self.uniforms[2],
                1,
                gl::FALSE,
                cam.get_view_projection().as_ptr(),
            );
        }
    }
}
