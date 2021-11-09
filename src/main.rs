mod vec3;
mod draw;
mod ray;
mod shape;
mod hit;
mod hittable_list;
mod camera;
mod common;
mod material;

use std::fmt::{Display, Formatter};
use crate::vec3::Vec3;
use std::{thread};
use crate::draw::write_color;
use std::borrow::{BorrowMut, Borrow};
use crate::ray::{Point3, Ray};
use crate::shape::{ Sphere};
use crate::hit::{HitRecorder, Hittable};
use crate::hittable_list::HittableList;
use std::sync::{Arc, mpsc};
use crate::camera::Camera;
use crate::common::{rand_f64, clamp};
use crate::material::{Lambertian, Metal};

type Color = Vec3;


impl Display for Color{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{} {} {}\n",
               (256.0 * clamp(self.x, 0.0, 0.999)) as i32,
               (256.0 * clamp(self.y, 0.0, 0.999)) as i32,
               (256.0 * clamp(self.z, 0.0, 0.999)) as i32)
    }
}
impl Color{
    pub(crate) fn set(r:f64,g:f64,b:f64) -> Self{
        Vec3{
            x:r,
            y:g,
            z:b,
        }
    }
}

fn ray_color(ray:Ray,world:&HittableList,depth:i32) -> Color{
    let mut rec = HitRecorder::new();
    if depth <= 0 {
        return Color::set(0.0,0.0,0.0);
    }
    if world.hit(ray, 0.001, f64::MAX, rec.borrow_mut()){
        let attenuation = rec.material.clone().unwrap().get_color();
        let ray = rec.material.clone().unwrap().scatter(&ray, rec);
        return match ray {
            Some(scattered) => {
                attenuation * ray_color(scattered, world, depth - 1)
            }
            None => { Color::set(0.0, 0.0, 0.0) }
        }
        // let target = rec.p.unwrap() + Vec3::random_in_hemisphere(rec.normal.unwrap());
        // return ray_color(Ray::form(rec.p.unwrap(),target- rec.p.unwrap()),world.borrow(),depth-1) * 0.5;
    }
    let unit_direction = ray.direction().unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    Color::set(1.0,1.0,1.0) * (1.0 - t ) + Color::set(0.5,0.7,1.0) * t
}

fn main() {
    //Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    //Materials
    let m_ground = Arc::new(Lambertian::form(0.8,0.8,0.0));
    let m_center = Arc::new(Lambertian::form(0.7,0.3,0.3));
    let m_left= Arc::new(Metal::form(0.8,0.8,0.8,0.3));
    let m_right= Arc::new(Metal::form(0.8,0.6,0.2,0.3));



    //World
    let mut world =HittableList::new();
    world.add(Arc::new(Sphere::form(Point3::form(0.0,-100.5,-1.0),100.0,m_ground)));
    world.add(Arc::new(Sphere::form(Point3::form(0.0,0.0,-1.0),0.5,m_center)));
    world.add(Arc::new(Sphere::form(Point3::form(-1.0,0.0,-1.0),0.5,m_left)));
    world.add(Arc::new(Sphere::form(Point3::form(1.0,0.0,-1.0),0.5,m_right)));


    let world_arc = Arc::new(world);
    //Camera
    let camera_arc = Arc::new(Camera::new(Point3::new(),2.0));
    let count = 10; //图形渲染线程数
    let (tx, rx) = mpsc::channel();
    for thread_n in  0 .. count{
        let camera_t = camera_arc.clone();
        let world_t = world_arc.clone();
        let chan = tx.clone();
        let _t = thread::spawn(move ||{
            let mut file = std::fs::File::create(format!("file{}",thread_n)).expect("create failed");
            let per_num = (image_height as f32 / count as f32).ceil() as i32;
            let render_segment_start = per_num * thread_n;
            let mut render_segment_end =  per_num * (thread_n + 1);
            if render_segment_end > image_height{
                render_segment_end = image_height
            }
            for  j in render_segment_start .. render_segment_end{
                for  i in 0.. image_width {
                    let mut pixel_color = Color::new();
                    for _s in  0 .. samples_per_pixel{
                        let u = (i as f64 + rand_f64()) / (image_width -1) as f64;
                        let v = (((image_height - 1) - j)  as f64 + rand_f64()) / (image_height - 1) as f64 ;
                        let ray = camera_t.get_ray(u,v);
                        pixel_color += ray_color(ray,world_t.borrow(),max_depth);
                    }
                    write_color(file.borrow_mut(), pixel_color,samples_per_pixel)
                }
            }
            let _ = chan.send(());
        });
    }
    for _ in 0..count {
        let _ = rx.recv();
    }


}
