use std::io::Cursor;
use glium::{Display, Frame};
use crate::{Camera, Entity, Input, Material, Mesh, Shader, Transform};

pub struct Game{
    pub camera:Camera,
    pub input_system:Input,
    pub entities:Vec<Entity>
}

impl Game{
    pub fn new() -> Game {
        Game{
            camera: Camera::new(),
            input_system:Input::new(),
            entities: Vec::new(),
        }
    }

    pub fn init(&mut self, display: &Display){
        let image = image::load(Cursor::new(&include_bytes!("../../res/dirt.jpg")), image::ImageFormat::Jpeg).unwrap().to_rgba8();
        let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        let diffuse_tex = glium::texture::SrgbTexture2d::new(display, image).unwrap();

        let mut entity = Entity::new(
            Mesh::quad(&display),
            Transform::new(),
            Material{
                shader: Shader::new(display, "vertex.glsl", "fragment.glsl"),
                diff_tex:diffuse_tex});
        entity.transform.position.z += 5.0;
        self.add_new_entity(entity);

    }

    pub fn update(&mut self, delta_time:f32, locked_cursor: bool){
        self.camera.update(&self.input_system, delta_time, locked_cursor);
    }

    pub fn render(&mut self, target:&mut Frame){
        for i in 0..self.entities.len(){
            self.entities[i].render(target, &self.camera);
        }
    }

    pub fn add_new_entity(&mut self, entity: Entity){
        self.entities.push(entity);
    }


}







