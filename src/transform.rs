extern crate glm;
extern crate num;

use super::camera::Camera;

pub struct Transform {
    pos: glm::Vec3,
    rot: glm::Vec3,
    scale: glm::Vec3,
}

impl Transform {
    pub fn get_model(&self) -> glm::Mat4 {
        let pos_mat = glm::ext::translate(&num::one(), self.pos);
        let scale_mat = glm::ext::scale(&num::one(), self.scale);
        let rot_x = glm::ext::rotate(&num::one(), self.rot.x, glm::vec3(1.0, 0.0, 0.0));
        let rot_y = glm::ext::rotate(&num::one(), self.rot.y, glm::vec3(0.0, 1.0, 0.0));
        let rot_z = glm::ext::rotate(&num::one(), self.rot.z, glm::vec3(0.0, 0.0, 1.0));
        let rot_mat = rot_x * rot_y * rot_z;
        pos_mat * rot_mat * scale_mat
    }
    pub fn get_mvp(&self, cam: &Camera) -> glm::Mat4 {
        let vp = cam.get_view_projection();
        let m = self.get_model();
        vp * m
    }
}
