use glium::{Display, VertexBuffer};
use crate::Vertex;

pub struct Mesh{
    pub shape:Vec<Vertex>,
    pub vertex_buffer:VertexBuffer<Vertex>,
    pub indices : glium::IndexBuffer<u16>
}

impl Mesh{
    pub fn new(display: &Display) -> Mesh{

        let shape = Vec::new();
        let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();
        let indices = glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &[]).unwrap();

        Mesh{
            shape,
            vertex_buffer,
            indices
        }
    }

    pub fn from_vertex_list(display: &Display, shape: Vec<Vertex>, indices:Vec<u16>) -> Mesh{

        let vertex_buffer =  glium::VertexBuffer::new(display, &shape).unwrap();
        let indices = glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &indices).unwrap();

        Mesh{
            shape,
            vertex_buffer,
            indices
        }
    }

    pub fn quad(display: &Display) -> Mesh{

        let vertex1 = Vertex { position: [-0.5, -0.5, 0.0], tex_coords: [0.0, 0.0]};
        let vertex2 = Vertex { position:[ 0.5,  -0.5, 0.0], tex_coords: [1.0, 0.0]};
        let vertex3 = Vertex { position:[ -0.5,  0.5, 0.0], tex_coords: [0.0, 1.0]};
        let vertex4 = Vertex { position: [ 0.5, 0.5, 0.0] , tex_coords: [1.0, 1.0]};

        let indices:[u16; 6]=[
            0, 1, 2,
            1, 2, 3
        ];



        let shape = vec![vertex1, vertex2, vertex3, vertex4];
        let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();
        let indices = glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &indices).unwrap();
        Mesh{
            shape,
            vertex_buffer,
            indices
        }
    }

    pub fn cube(display: &Display) -> Mesh{

        let vertex0 = Vertex { position: [-0.5, 0.5, -0.5], tex_coords: [0.0, 0.0] };
        let vertex1 = Vertex { position: [ 0.5, 0.5, -0.5], tex_coords: [1.0, 0.0] };
        let vertex2 = Vertex { position: [-0.5, 0.5, 0.5], tex_coords: [0.0, 1.0] };
        let vertex3 = Vertex { position: [ 0.5, 0.5, 0.5], tex_coords: [1.0, 1.0] };

        let vertex4 = Vertex { position: [-0.5, -0.5, 0.0], tex_coords: [0.0, 0.0] };
        let vertex5 = Vertex { position: [ 0.5,  -0.5, 0.0], tex_coords: [1.0, 0.0] };
        let vertex6 = Vertex { position: [ -0.5,  0.5, 0.0], tex_coords: [0.0, 1.0] };
        let vertex7 = Vertex { position: [ 0.5, 0.5, 0.0], tex_coords: [1.0, 1.0] };

        let indices:[u16; 36]=[
            0, 1, 2,
            1, 2, 3,//top

            4, 5, 6,
            5, 6, 7,//bottom

            4, 5, 0,
            5, 0, 1,//front

            6, 4, 2,
            4, 2, 0,//left

            5, 7, 1,
            7, 1, 3,//right

            7, 6, 3,
            6, 3, 2//back

        ];



        let shape = vec![vertex0, vertex1, vertex2, vertex3, vertex4, vertex5, vertex6, vertex7];
        let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();
        let indices = glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &indices).unwrap();
        Mesh{
            shape,
            vertex_buffer,
            indices
        }
    }
}