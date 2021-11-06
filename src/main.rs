mod vec3;
mod draw;
mod ray;
mod shape;
mod hit;
mod hittable_list;
mod camera;
mod common;

use std::fmt::{Display, Formatter};
use crate::vec3::Vec3;
use std::io;
use crate::draw::write_color;
use std::borrow::{BorrowMut, Borrow};
use crate::ray::{Point3, Ray};
use std::io::{Write};
use crate::shape::{ Sphere};
use crate::hit::{HitRecorder, Hittable};
use crate::hittable_list::HittableList;
use std::sync::Arc;
use crate::camera::Camera;
use crate::common::{rand_f64, clamp};

type Color = Vec3;

impl Display for Color{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{} {} {}\n",(255.999 * clamp(self.x, 0.0, 0.999)) as i32,
               (255.999 * clamp(self.y, 0.0, 0.999)) as i32,
               (255.999 * clamp(self.z, 0.0, 0.999)) as i32)
    }
}

fn ray_color(ray:Ray,world:&HittableList) -> Color{
    let mut rec = HitRecorder::new();
    if world.hit(ray,0.0,f64::MAX,rec.borrow_mut()){
        return  (rec.normal.unwrap() + Color::form(1.0,1.0,1.0)) * 0.5;
    }
    let unit_direction = ray.direction().unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    Color::form(1.0,1.0,1.0) * (1.0 - t ) + Color::form(0.5,0.7,1.0) * t
}

fn main() {
    //Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;

    //World
    let mut world = HittableList::new();
    world.add(Arc::new(Sphere::form(Point3::form(0.0,0.0,-1.0),0.5)));
    world.add(Arc::new(Sphere::form(Point3::form(0.0,-100.5,-1.0),100.0)));

    //Camera
    let camera = Camera::new(Point3::new(),2.0);

    let mut stdout = io::stdout();
    let _ = stdout.write(format!("P3\n {} {}\n255\n",image_width,image_height).as_bytes());
    for  j in 0 .. image_height {
        for  i in 0.. image_width {
            let mut pixel_color = Color::new();
            //像素渲染抗齿距处理
            for s in  0 .. samples_per_pixel{
                let u = (i as f64 + rand_f64()) / (image_width -1) as f64;
                let v = (((image_height - 1) - j)  as f64 + rand_f64()) / (image_height - 1) as f64 ;
                pixel_color += ray_color(camera.get_ray(u,v),world.borrow());
            }
            write_color(stdout.borrow_mut(), pixel_color,samples_per_pixel)
        }
    }
}
