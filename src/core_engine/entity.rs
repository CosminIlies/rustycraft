use glium::{Frame, Surface, uniform};

use crate::core_engine::camera;
use crate::{Camera, Material, Mesh, Transform};


pub struct Entity{
    pub mesh: Mesh,
    pub transform: Transform,
    pub material: Material,
}

impl Entity{

    pub fn new(mesh: Mesh, transform:Transform, material:Material)->Entity{
        Entity{
            mesh, transform, material
        }
    }

    pub fn render(&self, target: &mut Frame, camera: &Camera){
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            .. Default::default()
        };

        target.draw(&self.mesh.vertex_buffer, &self.mesh.indices, &self.material.shader.program, &uniform!{
            view_mat:   camera.view_matrix().as_array(),
            proj_mat:   camera::projection_matrix(&target),
            trans_mat:  self.transform.transformation_matrix().as_array(),
            difuse_tex: &self.material.diff_tex
        },&params).unwrap();
    }

}