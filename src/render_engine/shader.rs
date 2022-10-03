use std::fs;
use glium::Display;

pub struct Shader{
    pub program:glium::Program
}

impl Shader{
    pub fn new(display: &Display, vert_file_name:&str, frag_file_name:&str) ->Shader{
        let vertex_file = fs::read_to_string( "res/Shaders/".to_owned() + vert_file_name).expect("Cannot open the vertex file");
        let fragment_file = fs::read_to_string("res/Shaders/".to_owned() + frag_file_name).expect("Cannot open the fragment file");

       Shader{
            program: glium::Program::from_source(display, vertex_file.as_str(), fragment_file.as_str(), None).unwrap()
       }


    }
}