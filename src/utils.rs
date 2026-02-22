use std::time::Duration;
use crate::components::Component;

#[derive(Default, Debug)]
pub struct Delta(Duration);

impl Delta {
    pub fn change(&mut self, duration: Duration) {
        self.0 = duration;
    }
    
    pub fn as_secs_f64(&self) -> f64 {
        self.0.as_secs_f64()
    }

    pub fn as_secs_f32(&self) -> f32 {
        self.0.as_secs_f32()
    }

    pub fn as_secs(&self) -> u64 {
        self.0.as_secs()
    }
    
    pub fn as_millis(&self) -> u128 {
        self.0.as_millis()
    }

    pub fn as_micros(&self) -> u128 {
        self.0.as_micros()
    }

    pub fn as_nanos(&self) -> u128 {
        self.0.as_nanos()
    }
}

impl Component for Delta {}