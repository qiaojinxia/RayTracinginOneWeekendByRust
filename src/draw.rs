use std::io::Write;
use crate::{Color};
use std::fs::File;

pub(crate) fn write_color(std: &mut File, color:Color,samples_per_pixel:i32){
    let mut r = color.x;
    let mut g = color.y;
    let mut b = color.z;

    let scale = 1.0 / samples_per_pixel as f64 ;

    r =(scale * r).sqrt();
    g =(scale * g).sqrt();
    b =(scale * b).sqrt();


    let s = format!("{}",Color::form(r,g,b));

    let _ = std.write(s.as_bytes());
}