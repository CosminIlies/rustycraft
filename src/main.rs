extern crate glium;
mod render_engine;
mod core_engine;
mod game;

use std::time::SystemTime;

use render_engine::vertex::Vertex;
use render_engine::shader::Shader;
use render_engine::material::Material;

use core_engine::mesh::Mesh;
use core_engine::entity::Entity;
use core_engine::transform::Transform;
use core_engine::camera;

use crate::glium::Surface;
use glium::{glutin};
use crate::glutin::dpi::{LogicalPosition};

use crate::camera::Camera;
use crate::core_engine::game::Game;
use crate::core_engine::input::Input;
use crate::core_engine::math::vec2::Vector2;



fn main(){
    
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb,cb,&event_loop).unwrap();

    let mut game = Game::new();
    game.init(&display);

    let mut locked_cursor = true;

    let mut delta_time = 0.0;
    let now = SystemTime::now();
    let mut begin_time = 0;
    let mut end_time =  0;


    event_loop.run(move |ev, _, control_flow|{

        if game.input_system.keys[glutin::event::VirtualKeyCode::Escape as usize] {
            locked_cursor = !locked_cursor;
            // *control_flow = glutin::event_loop::ControlFlow::Exit;
            // return;
        }



        game.update(delta_time,locked_cursor);

        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);
        game.render(&mut target);
        target.finish().unwrap();



        end_time = now.elapsed().unwrap().as_nanos();
        delta_time = end_time as f32 - begin_time as f32;
        delta_time /= 1e9;
        begin_time = end_time;



        //events
        if locked_cursor{
            display.gl_window().window().set_cursor_position( LogicalPosition::new(500,500)).expect("TODO: panic message");
        }

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
