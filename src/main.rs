mod vec3;
mod draw;
mod ray;
mod shape;

use std::fmt::{Display, Formatter};
use crate::vec3::Vec3;
use std::io;
use crate::draw::write_color;
use std::borrow::BorrowMut;
use crate::ray::{point3, Ray};
use std::io::Write;
use crate::shape::hit_sphere;

type Color = Vec3;

impl Display for Color{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{} {} {}\n",(255.999 * self.x) as i32, (255.999 * self.y) as i32, (255.999 *self.z) as i32)
    }
}

fn ray_color(ray:Ray) -> Color{
    if hit_sphere(point3{x :0.0 ,y : 0.0, z : -1.0}, 0.5, ray) {
        return Color{x : 1.0, y : 0.0, z : 0.0}
    }
    let unit_direction = ray.direction().unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    Color{x:1.0, y:1.0, z:1.0} * (1.0-t) + Color{x:0.5, y:0.7, z:1.0} * t
}

fn main() {
    //Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    //Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = point3{ x: 0.0, y: 0.0, z: 0.0 };
    let horizontal = Vec3{ x: viewport_width, y: 0.0, z: 0.0 };
    let vertical = Vec3{x: 0.0,y :viewport_height,z :0.0};
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3{x : 0.0,y : 0.0,z : focal_length};

    let mut stdout = io::stdout();
    stdout.write(format!("P3\n {} {}\n255\n",image_width,image_height).as_bytes());
    for  j in 0 .. image_height {
        for  i in 0.. image_width {
            let u = i as f64 / (image_width -1) as f64;
            let v = ((image_height - 1) - j)  as f64 / (image_height - 1) as f64 ;
            let ray = Ray::form(origin,lower_left_corner +  horizontal * u + vertical * v - origin);
            let pixel_color = ray_color(ray);
            write_color(stdout.borrow_mut(), pixel_color)
        }
    }
}
