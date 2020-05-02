extern crate cgmath;

use cgmath::vec3;
use cgmath::Matrix4;
use cgmath::Point3;
use cgmath::Vector3;

pub struct Camera {
    pub perspective: Matrix4<f32>,
    pub position: Point3<f32>,
    pub forward: Vector3<f32>,
    pub up: Vector3<f32>,
}

impl Camera {
    pub fn new(pos: Point3<f32>, fov: f32, aspect: f32, z_near: f32, z_far: f32) -> Camera {
        let camera = Camera {
            perspective: cgmath::perspective(cgmath::Rad(fov.to_radians()), aspect, z_near, z_far),
            position: pos,
            forward: vec3(0.0, 0.0, 1.0),
            up: vec3(0.0, 1.0, 0.0),
        };
        camera
    }
    pub fn get_view_projection(&self) -> Matrix4<f32> {
        self.perspective * Matrix4::look_at(self.position, self.position + self.forward, self.up)
    }
}
