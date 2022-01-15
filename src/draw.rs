use std::io::Write;
use crate::{Color};
use std::fs::File;

pub(crate) fn write_color( color:Color,samples_per_pixel:i32) -> Color{
    let mut r = color.x;
    let mut g = color.y;
    let mut b = color.z;
    if r == f64::NAN{
        r = 0.0
    }
    if g == f64::NAN{
        g = 0.0
    }
    if b == f64::NAN{
        b = 0.0
    }
    let scale = 1.0 / samples_per_pixel as f64 ;

    r =(scale * r).sqrt();
    g =(scale * g).sqrt();
    b =(scale * b).sqrt();

    Color::set(r,g,b)

    // let s = format!("{}",Color::form(r,g,b));
    //
    // let _ = std.write(s.as_bytes());
}