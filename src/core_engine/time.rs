use std::time::SystemTime;

pub struct Time{
    pub delta_time:f32,
    start_time:f32,
    end_time:f32,
    timer:SystemTime
}


impl Time{
    pub fn Time() ->Time{
        Time{
            delta_time: 0.0,
            start_time: 0.0,
            end_time: 0.0,
            timer: SystemTime::now()
        }
    }

    pub fn end_frame(&mut self){
        self.end_time = self.timer.elapsed().unwrap().as_secs() as f32;

    }

}