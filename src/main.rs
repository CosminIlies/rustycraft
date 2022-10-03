extern crate glium;
mod render_engine;
mod core_engine;

use std::collections::hash_map::Entry;
use std::io::Cursor;

use render_engine::vertex::Vertex;
use render_engine::shader::Shader;
use render_engine::material::Material;

use core_engine::mesh::Mesh;
use core_engine::entity::Entity;
use core_engine::transform::Transform;
use core_engine::camera;

use crate::glium::Surface;
use glium::{glutin, uniform};
use glium::glutin::dpi::Position;
use crate::glutin::dpi::{LogicalPosition, Size};

use crate::camera::Camera;
use crate::core_engine::game::Game;
use crate::core_engine::input::Input;
use crate::core_engine::math::vec3::Vector3;
use crate::core_engine::math::vec2::Vector2;



fn main(){
    
    let mut event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb,cb,&event_loop).unwrap();

    let mut game = Game::new();
    game.init(&display);

    let mut t:f32 = 0.0;


    event_loop.run(move |ev, _, control_flow|{
        t+=0.01;

        if game.input_system.keys[glutin::event::VirtualKeyCode::Escape as usize] {
            *control_flow = glutin::event_loop::ControlFlow::Exit;
            return;
        }



        game.update(0.0);

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        game.render(&mut target);
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
                    game.input_system.update_input(input);

                },
                glutin::event::WindowEvent::CursorMoved {position, ..} =>{
                    game.input_system.update_delta_mouse_pos(position.x as f32, position.y as f32);

                }
                _ => return,
            },
            _ => (),
        }


    });

}
