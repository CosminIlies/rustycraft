use crate::core_engine::math::quaternion::Quaternion;
use crate::core_engine::math::vec3::Vector3;
use crate::core_engine::math::mat4::Matrix4;

pub struct Transform{
    pub position:Vector3,
    pub rotation:Vector3,
    pub scale:Vector3,
}

impl Transform{
    pub fn new()-> Transform{
        Transform{
            position: Vector3::new(0.0, 0.0, 0.0),
            rotation: Vector3::new(0.0, 0.0, 0.0),
            scale: Vector3::new(1.0,1.0,1.0),
        }
    }

    pub fn transformation_matrix(&self) -> Matrix4{
        let mut mat = Matrix4::identity();


        mat.multiply_by(&self.rotation.to_quaternion().to_matrix() );

        mat.multiply_by(&Matrix4::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [self.position.x, self.position.y, self.position.z, 1.0],

        ]));


        mat.multiply_by(&Matrix4::new([
            [self.scale.x, 0.0, 0.0, 0.0],
            [0.0, self.scale.y, 0.0, 0.0],
            [0.0, 0.0, self.scale.z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]));


        mat
    }
}