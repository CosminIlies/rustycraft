use crate::core_engine::math::mat4::Matrix4;
use crate::core_engine::math::vec3::Vector3;

pub struct Quaternion{
    pub x:f32,
    pub y:f32,
    pub z:f32,
    pub w:f32,
}

impl Quaternion{
    pub fn new(x:f32,y:f32,z:f32,w:f32,)->Quaternion{
        Quaternion{x, y, z, w}
    }

    pub fn rotate_euler_angle(&mut self, euler: Vector3){
        let cy = (euler.y * 0.5).cos();
        let sy = (euler.y * 0.5).sin();

        let cp = (euler.x * 0.5).cos();
        let sp = (euler.x * 0.5).sin();

        let cr = (euler.z * 0.5).cos();
        let sr = (euler.z * 0.5).sin();

        self.w = cr * cp * cy + sr * sp * sy;
        self.x = sr * cp * cy - cr * sp * sy;
        self.y = cr * sp * cy + sr * cp * sy;
        self.z = cr * cp * sy - sr * sp * cy;

    }

    pub fn to_matrix(&self) -> Matrix4{
        let qx2 = self.x * self.x;
        let qy2 = self.y * self.y;
        let qz2 = self.z * self.z;

        let qx = self.x;
        let qy = self.y;
        let qz = self.z;
        let qw = self.w;

        Matrix4::new([
            [1.0-2.0*qy2-2.0*qz2, 2.0*qx*qy - 2.0*qz*qw, 2.0*qx*qz + 2.0*qy*qw, 0.0],
            [2.0*qx*qy+2.0*qz*qw,   1.0-2.0*qx2-2.0*qz2, 2.0*qy*qz - 2.0*qx*qw, 0.0],
            [2.0*qx*qz-2.0*qy*qw, 2.0*qy*qz+2.0*qx*qw,   1.0-2.0*qx2 - 2.0*qy2, 0.0],
            [0.0,            0.0,              0.0,               1.0],
        ])
    }
}