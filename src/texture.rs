use crate::Color;
use crate::ray::Point3;

trait Texture {
    fn value(u:f64,v:f64,p:&Point3) -> Color;
}

pub(crate) struct Solid{
    color_value:Color,
}

impl Solid{
    fn form(r:f64,g:f64,b:f64) -> Self{
        Solid{
            color_value: Color::form(r,g,b)
        }
    }
    fn form_color(c:Color) -> Self{
        Solid{
            color_value: c,
        }
    }
}

impl Texture for Solid{
    fn value(u: f64, v: f64, p: &Point3) -> Color {
        todo!()
    }
}