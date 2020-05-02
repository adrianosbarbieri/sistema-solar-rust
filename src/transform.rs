extern crate cgmath;
extern crate num;

use cgmath::Matrix4;
use cgmath::Rad;
use cgmath::Vector3;

pub struct Transform {
    pub pos: Vector3<f32>,
    pub rot: Vector3<f32>,
    pub scale: Vector3<f32>,
    pub scale_val: f32,
}

impl Transform {
    pub fn get_model(&self) -> Matrix4<f32> {
        let pos_mat = Matrix4::<f32>::from_translation(self.pos);
        let scale_mat = Matrix4::<f32>::from_scale(self.scale_val);
        let rot_x = Matrix4::<f32>::from_angle_x(Rad(self.rot.x));
        let rot_y = Matrix4::<f32>::from_angle_x(Rad(self.rot.y));
        let rot_z = Matrix4::<f32>::from_angle_x(Rad(self.rot.z));
        let rot_mat = rot_x * rot_y * rot_z;
        pos_mat * rot_mat * scale_mat
    }
}
