use glium::{Frame, Surface};
use crate::core_engine::math::mat4::Matrix4;
use crate::core_engine::math::quaternion::Quaternion;
use crate::Transform;

pub struct Camera{
    pub transform:Transform
}

impl Camera{
    pub fn new()->Camera{
        Camera{
            transform: Transform::new()
        }
    }

    pub fn view_matrix(&self) -> Matrix4{
        let mut mat = Matrix4::identity();

        mat.multiply_by(&Matrix4::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [-self.transform.position.x, -self.transform.position.y, -self.transform.position.z, 1.0],

        ]));

        mat.multiply_by(&self.transform.rotation.to_quaternion().to_matrix());


        mat
    }
}





pub fn projection_matrix(target: &Frame) -> [[f32; 4]; 4]{
    let (width, height) = target.get_dimensions();
    let aspect_ratio = height as f32 / width as f32;

    let fov: f32 = 3.141592 / 3.0;
    let zfar = 1024.0;
    let znear = 0.1;

    let f = 1.0 / (fov / 2.0).tan();

    [
        [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
        [         0.0         ,     f ,              0.0              ,   0.0],
        [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
        [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
    ]
}