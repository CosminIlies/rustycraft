extern crate glium;
mod render_engine;
mod core_engine;

use std::io::Cursor;
use render_engine::vertex::Vertex;
use render_engine::shader::Shader;
use core_engine::mesh::Mesh;
use core_engine::transform::Transform;
use core_engine::camera;

use crate::glium::Surface;
use glium::{glutin, uniform};
use glium::glutin::dpi::Position;
use crate::camera::Camera;
use crate::core_engine::math::vec3::Vector3;
use crate::core_engine::math::vec2::Vector2;
use crate::glutin::dpi::{LogicalPosition, Size};


fn main(){
    
    let mut event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb,cb,&event_loop).unwrap();


    let mut camera = Camera::new();
    let mesh = Mesh::quad(&display);
    let mut transform = Transform::new();
    transform.position.z = 5.0;
    let shader = Shader::new(&display, "vertex.glsl", "fragment.glsl");

    let image = image::load(Cursor::new(&include_bytes!("../res/dirt.jpg")), image::ImageFormat::Jpeg).unwrap().to_rgba8();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let diffuse_tex = glium::texture::SrgbTexture2d::new(&display, image).unwrap();

    let mut t:f32 = 0.0;
    let mut keys:[bool; 256] = [false; 256];


    let mut dx:f32 = 0.0;
    let mut dy:f32 = 0.0;


    event_loop.run(move |ev, _, control_flow|{
        t+=0.01;
        let mut dir = Vector2::new(0.0, 0.0);
        let speed = 0.01;

        if keys[glutin::event::VirtualKeyCode::W as usize]{
            dir.y += 1.0;
        }
        if keys[glutin::event::VirtualKeyCode::S as usize]{
            dir.y -= 1.0;
        }
        if keys[glutin::event::VirtualKeyCode::A as usize]{
            dir.x -= 1.0;
        }
        if keys[glutin::event::VirtualKeyCode::D as usize]{
            dir.x += 1.0;
        }
        dir.normailze();

        let sn = -camera.transform.rotation.x.sin();
        let cs = camera.transform.rotation.x.cos();

        camera.transform.position.x += (dir.x * cs - dir.y * sn) * speed;
        camera.transform.position.z += (dir.x * sn + dir.y * cs) * speed;

        camera.transform.rotation.z -= dy / 1000.0;
        camera.transform.rotation.x -= dx / 1000.0;


        if keys[glutin::event::VirtualKeyCode::Escape as usize] {
            *control_flow = glutin::event_loop::ControlFlow::Exit;
            return;
        }





        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.draw(&mesh.vertex_buffer, &mesh.indices, &shader.program, &uniform!{
            view_mat:   camera.view_matrix().as_array(),
            proj_mat:   camera::projection_matrix(&target),
            trans_mat:  transform.transformation_matrix().as_array(),
            difuse_tex: &diffuse_tex
        },
                    &Default::default()).unwrap();
        target.finish().unwrap();








        //events


        display.gl_window().window().set_cursor_position( LogicalPosition::new(500,500)).expect("TODO: panic message");

        let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                glutin::event::WindowEvent::KeyboardInput {input, ..} => {
                    if input.state != glutin::event::ElementState::Released{
                        let key = input.virtual_keycode.unwrap() as usize;
                        keys[key] = true;
                    }else{
                        let key = input.virtual_keycode.unwrap() as usize;
                        keys[key] = false;
                    }
                },
                glutin::event::WindowEvent::CursorMoved {position, ..} =>{
                    dx = 500.0 - position.x as f32;
                    dy = 500.0 - position.y as f32;
                }
                _ => return,
            },
            _ => (),
        }




    });

}
