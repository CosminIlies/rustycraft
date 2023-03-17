use std::fs::File;
use std::io::BufReader;
use glium::Display;
use crate::{Mesh, Vertex};

#[derive(Copy,Clone)]
pub struct Block{
    id:u16,
}

pub struct Chunk{
    pub blocks:[[[Block; 16]; 16]; 16],
    pub mesh: Mesh
}

impl Chunk{
    pub fn new(display: &Display) -> Chunk{
        Chunk{
            blocks: [[[Block{ id: 1 }; 16]; 16]; 16],
            mesh: Mesh::new(display),
        }
    }

    pub fn from_file(path: &str,display: &Display) -> Chunk{

        // let file = File::open(path).expect("File not found");
        // let reader = BufReader::new(file);

        let mut blocks:[[[Block; 16]; 16]; 16]  = [[[Block{ id: 0 }; 16]; 16]; 16];

        for x in 0..16{
            for y in 0..16 {
                for z in 0..16 {
                    if y == 15{
                        blocks[8][y][8].id = 1;
                    }else{
                        if y < 10{
                            blocks[x][y][z].id = 1;
                        }else if x < z{
                            blocks[x][y][z].id = 1;
                        }
                        if x == 0{
                            blocks[x][y][z].id = 1;
                        }
                        if z == 0{
                            blocks[x][y][z].id = 1;
                        }
                    }



                }
            }
        }
        Chunk{
            blocks,
            mesh: Mesh::new(display),
        }
    }

    pub fn update(&mut self, display:&Display){

        self.gen_mesh(display);
    }


    fn gen_mesh(&mut self, display: &Display){
        let mut shape:Vec<Vertex> = Vec::new();

        shape.append(&mut Mesh::cube(display, [true, false, false, false, false, false], 0.0, 0.0, 0.0));



        for x in 0..16{
            for y in 0..16 {
                for z in 0..16 {
                    if self.blocks[x][y][z].id == 1 {
                        let mut faces: [bool; 6] = [false; 6];

                        if x == 0 {
                            faces[4] = true;
                        }else if self.blocks[x - 1][y][z].id == 0{
                            faces[4] = true;
                        }

                        if x == 15 {
                            faces[1] = true;
                        }else if self.blocks[x + 1][y][z].id == 0{
                            faces[1] = true;
                        }

                        if z == 0 {
                            faces[0] = true;
                        }else if self.blocks[x][y][z-1].id == 0{
                            faces[0] = true;
                        }

                        if z == 15 {
                            faces[3] = true;
                        }else if self.blocks[x][y][z+1].id == 0{
                            faces[3] = true;
                        }

                        if y == 0 {
                            faces[5] = true;
                        }else if self.blocks[x][y-1][z].id == 0{
                            faces[5] = true;
                        }

                        if y == 15 {
                            faces[2] = true;
                        }else if self.blocks[x][y+1][z].id == 0{
                            faces[2] = true;
                        }

                        shape.append(&mut Mesh::cube(display, faces, x as f32, y as f32, z as f32));
                    }
                }
            }
        }


        let mut indices: Vec<u16> = Vec::new();
        for _ in 0..shape.len()/4 {
            let face = indices.len() / 6;
            indices.push((0 + face * 4) as u16);
            indices.push((1 + face * 4) as u16);
            indices.push((2 + face * 4) as u16);

            indices.push((2 + face * 4) as u16);
            indices.push((1 + face * 4) as u16);
            indices.push((3 + face * 4) as u16);

        }

        self.mesh = Mesh::from_vertex_list(display, shape, indices);

    }

}