use crate::core_engine::math::quaternion::Quaternion;

pub struct Vector3{
    pub x:f32,
    pub y:f32,
    pub z:f32
}

impl Vector3{

    pub fn new(x:f32, y:f32, z:f32) -> Vector3{
        Vector3{ x , y, z }
    }

    pub fn to_quaternion(&self) -> Quaternion{
        let mut rot = Quaternion::new(0.0, 0.0, 0.0, 0.0);
        rot.rotate_euler_angle(Vector3::new(self.x, self.y, self.z));
        rot
    }
}