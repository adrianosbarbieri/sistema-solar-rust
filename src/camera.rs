extern crate glm;

pub struct Camera {
    perspective: glm::Mat4,
    position: glm::Vec3,
    forward: glm::Vec3,
    up: glm::Vec3,
}

pub fn new_camera(pos: glm::Vec3, fov: f32, aspect: f32, z_near: f32, z_far: f32) -> Camera {
    let camera = Camera {
        perspective: glm::ext::perspective(fov, aspect, z_near, z_far),
        position: pos,
        forward: glm::vec3(0.0, 0.0, 1.0),
        up: glm::vec3(0.0, 1.0, 0.0),
    };
    camera
}

impl Camera {
    pub fn get_view_projection(&self) -> glm::Mat4 {
        self.perspective * glm::ext::look_at(self.position, self.position + self.forward, self.up)
    }
}
