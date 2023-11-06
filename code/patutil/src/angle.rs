use std::f32::consts::PI;




pub fn radians(degrees: f32) -> f32{
    degrees*PI/180. 
}

pub fn degrees(radians: f32) -> f32{
    radians*180./PI
}