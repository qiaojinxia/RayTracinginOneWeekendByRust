use std::io::Write;
use std::io;
use crate::{Color};

pub(crate) fn write_color(std: &mut io::Stdout, color:Color){
    let s = format!("{}",color);
    let _ = std.write(s.as_bytes());
}