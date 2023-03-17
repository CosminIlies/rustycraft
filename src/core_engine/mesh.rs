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
    /**

        front:0
        right:1
        up:2
        back:3
        left:4
        down:5

     */
    pub fn cube (display: &Display, faces:[bool; 6], x:f32, y:f32, z:f32) -> Vec<Vertex>{
        let mut shape:Vec<Vertex> = Vec::new();
        let mut indices:Vec<u16> = Vec::new();

        let mut nr_faces = 0;

        if faces[0]{
            nr_faces += 1;
            let vertex0 = Vertex { position: [x + -0.5, y + -0.5, z + -0.5], tex_coords: [0.0, 0.0] };
            let vertex1 = Vertex { position: [x + 0.5, y + -0.5, z + -0.5], tex_coords: [1.0, 0.0] };
            let vertex2 = Vertex { position: [x + -0.5, y + 0.5, z + -0.5], tex_coords: [0.0, 1.0] };
            let vertex3 = Vertex { position: [x + 0.5, y + 0.5, z + -0.5], tex_coords: [1.0, 1.0] };

            shape.push(vertex0);
            shape.push(vertex1);
            shape.push(vertex2);
            shape.push(vertex3);

        }

        if faces[1]{
            nr_faces += 1;
            let vertex0 = Vertex { position: [x + 0.5, y + -0.5, z + -0.5], tex_coords: [0.0, 0.0] };
            let vertex1 = Vertex { position: [x + 0.5, y + -0.5, z + 0.5], tex_coords: [1.0, 0.0] };
            let vertex2 = Vertex { position: [x + 0.5, y + 0.5, z + -0.5], tex_coords: [0.0, 1.0] };
            let vertex3 = Vertex { position: [x + 0.5, y + 0.5, z + 0.5], tex_coords: [1.0, 1.0] };

            shape.push(vertex0);
            shape.push(vertex1);
            shape.push(vertex2);
            shape.push(vertex3);

        }

        if faces[2]{
            nr_faces += 1;
            let vertex0 = Vertex { position: [x + -0.5, y + 0.5, z + -0.5], tex_coords: [0.0, 0.0] };
            let vertex1 = Vertex { position: [x + 0.5, y + 0.5, z + -0.5], tex_coords: [1.0, 0.0] };
            let vertex2 = Vertex { position: [x + -0.5, y + 0.5, z + 0.5], tex_coords: [0.0, 1.0] };
            let vertex3 = Vertex { position: [x + 0.5, y + 0.5, z + 0.5], tex_coords: [1.0, 1.0] };

            shape.push(vertex0);
            shape.push(vertex1);
            shape.push(vertex2);
            shape.push(vertex3);

        }
        if faces[3]{
            nr_faces += 1;
            let vertex0 = Vertex { position: [x + -0.5, y + -0.5, z + 0.5], tex_coords: [0.0, 0.0] };
            let vertex1 = Vertex { position: [x + 0.5, y + -0.5, z + 0.5], tex_coords: [1.0, 0.0] };
            let vertex2 = Vertex { position: [x + -0.5, y + 0.5, z + 0.5], tex_coords: [0.0, 1.0] };
            let vertex3 = Vertex { position: [x + 0.5, y + 0.5, z + 0.5], tex_coords: [1.0, 1.0] };

            shape.push(vertex0);
            shape.push(vertex2);
            shape.push(vertex1);
            shape.push(vertex3);

        }

        if faces[4]{
            nr_faces += 1;
            let vertex0 = Vertex { position: [x + -0.5, y + -0.5, z + -0.5], tex_coords: [0.0, 0.0] };
            let vertex1 = Vertex { position: [x + -0.5, y + -0.5, z + 0.5], tex_coords: [1.0, 0.0] };
            let vertex2 = Vertex { position: [x + -0.5, y + 0.5, z + -0.5], tex_coords: [0.0, 1.0] };
            let vertex3 = Vertex { position: [x + -0.5, y + 0.5, z + 0.5], tex_coords: [1.0, 1.0] };

            shape.push(vertex0);
            shape.push(vertex2);
            shape.push(vertex1);
            shape.push(vertex3);

        }

        if faces[5]{
            nr_faces += 1;
            let vertex0 = Vertex { position: [x + -0.5, y + -0.5, z + -0.5], tex_coords: [0.0, 0.0] };
            let vertex1 = Vertex { position: [x + 0.5, y + -0.5, z + -0.5], tex_coords: [1.0, 0.0] };
            let vertex2 = Vertex { position: [x + -0.5, y + -0.5, z + 0.5], tex_coords: [0.0, 1.0] };
            let vertex3 = Vertex { position: [x + 0.5, y + -0.5, z + 0.5], tex_coords: [1.0, 1.0] };

            shape.push(vertex0);
            shape.push(vertex2);
            shape.push(vertex1);
            shape.push(vertex3);
        }

        for i in 0..shape.len()/4 {
            let face = indices.len() / 6;
            indices.push((0 + face * 4) as u16);
            indices.push((1 + face * 4) as u16);
            indices.push((2 + face * 4) as u16);

            indices.push((2 + face * 4) as u16);
            indices.push((1 + face * 4) as u16);
            indices.push((3 + face * 4) as u16);

        }

        let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();
        let indices = glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &indices).unwrap();

        shape
        // Mesh{
        //     shape,
        //     vertex_buffer,
        //     indices
        // }
    }

}