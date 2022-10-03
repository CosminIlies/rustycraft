use crate::glutin;
use crate::glutin::event::KeyboardInput;

pub struct Input{
    pub keys:[bool; 256],
    pub dx:f32,
    pub dy:f32
}

impl Input{
    pub fn new()->Input{
        Input{
            keys:[false; 256],
            dx: 0.0,
            dy: 0.0
        }
    }

    pub fn update_input(&mut self,input:KeyboardInput){
        if input.state != glutin::event::ElementState::Released{
            let key = input.virtual_keycode.unwrap() as usize;
            self.keys[key] = true;
        }else{
            let key = input.virtual_keycode.unwrap() as usize;
            self.keys[key] = false;
        }
    }

    pub fn update_delta_mouse_pos(&mut self, x:f32, y:f32){
        self.dx = 500.0 - x;
        self.dy = 500.0 - y;
    }
}