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
        forward: glm::Vec3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
        up: glm::Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
    };
    camera
}

pub fn get_view_projection(camera: &mut Camera) -> glm::Mat4 {
    camera.perspective
        * glm::ext::look_at(camera.position, camera.position + camera.forward, camera.up)
}
