pub struct Vector2{
    pub x:f32,
    pub y:f32
}

impl Vector2{
    pub fn new(x:f32, y:f32) -> Vector2{
        Vector2{x, y}
    }

    pub fn normailze(&mut self){
        let mag = ((self.x*self.x + self.y*self.y) as f32).sqrt();
        if mag > 0.0{
            self.x /=  mag;
            self.y /= mag;
        }

    }
}