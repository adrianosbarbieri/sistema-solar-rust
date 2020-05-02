use image::*;
use std::ffi::c_void;

pub struct Texture {
    texture: u32,
}

pub unsafe fn init_texture(img: DynamicImage) -> u32 {
    let mut texture: u32 = 0;
    let (width, height) = img.dimensions();
    let real_img = match img {
        DynamicImage::ImageRgba8(img) => img,
        img => img.to_rgba(),
    };
    gl::GenTextures(1, &mut texture);
    gl::BindTexture(gl::TEXTURE_2D, texture);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
    gl::TexParameterf(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as f32);
    gl::TexParameterf(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as f32);
    let vec1 = real_img.into_raw();
    gl::TexImage2D(
        gl::TEXTURE_2D,
        0,
        gl::RGBA as i32,
        width as i32,
        height as i32,
        0,
        gl::RGBA,
        gl::UNSIGNED_BYTE,
        vec1.as_ptr() as *const c_void,
    );

    texture
}

impl Texture {
    pub fn new(file_name: &str) -> Texture {
        let res = image::open(file_name);
        let tex = match res {
            Err(s) => {
                println!("{}", s);
                0
            }
            Ok(img) => unsafe { init_texture(img) },
        };

        Texture { texture: tex }
    }
    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.texture);
        }
    }
    pub fn destroy(&self) {
        unsafe {
            gl::DeleteTextures(1, &self.texture);
        }
    }
}
