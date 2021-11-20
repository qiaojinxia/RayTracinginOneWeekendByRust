mod vec3;
mod draw;
mod ray;
mod shape;
mod hit;
mod hittable_list;
mod camera;
mod common;
mod material;
mod stl_reader;
mod bvh;
mod sort;
mod texture;
mod sences;

use std::fmt::{Display, Formatter};
use crate::vec3::Vec3;
use std::{ thread};
use crate::draw::write_color;
use std::borrow::{BorrowMut, Borrow};
use crate::ray::{Point3, Ray};
use crate::shape::{Sphere};
use crate::hit::{HitRecorder, Hittable};
use crate::hittable_list::HittableList;
use std::sync::{Arc, mpsc};
use crate::camera::Camera;
use crate::common::{rand_f64, clamp, rand_range_f64};
use crate::material::{Lambertian, Metal, Dielectric, Materials};
use crate::stl_reader::StlReader;
use crate::bvh::BvhNode;
use std::time::Instant;
use crate::texture::CheckerTexture;
use crate::sences::{two_spheres, random_scene, two_perlin_spheres, simple_light, cornell_box};

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

fn ray_color(ray:Ray,background:&Color,world:&HittableList,depth:i32) -> Color{
    let mut rec = HitRecorder::new();
    if depth <= 0 {
        return Color::set(0.0,0.0,0.0);
    }
    if world.hit(ray, 0.0001, f64::MAX, rec.borrow_mut()){
        let ray = rec.material.clone().unwrap().scatter(&ray, &mut rec);
        let emitted= rec.material.clone().unwrap().emitted(rec.u,rec.v,rec.p.unwrap());
        return match ray {
            Some(scattered) => {
                let attenuation = rec.material.clone().unwrap().get_color(&rec);
                emitted + attenuation * ray_color(scattered, background,world, depth - 1)
            }
                None => emitted
        }
        // let target = rec.p.unwrap() + Vec3::random_in_hemisphere(rec.normal.unwrap());
        // return ray_color(Ray::form(rec.p.unwrap(),target- rec.p.unwrap()),world.borrow(),depth-1) * 0.5;
    }
    // let unit_direction = ray.direction().unit_vector();
    // let t = 0.5 * (unit_direction.y + 1.0);
    // Color::set(1.0,1.0,1.0) * (1.0 - t ) + Color::set(0.5,0.7,1.0) * t
    *background
}



fn main() {
    let start = Instant::now();
    let mut lookfrom = Vec3::new();
    let mut lookat  = Vec3::new();
    let mut vfov = 40.0;
    let mut aperture = 0.0;
    let seneces = 6;
    let mut objs = vec![];
    let mut background= Color::form(0.0,0.0,0.0);

    //Image
    let mut image_width = 400;
    let mut samples_per_pixel = 100;
    let mut aspect_ratio = 16.0 / 9.0;

    match seneces {
        1 => {
            objs = random_scene();
            lookfrom = Point3::form(13.0,2.0,3.0);
            lookat = Point3::form(0.0,0.0,0.0);
            background= Color::form(0.70, 0.80, 1.00);
            vfov = 20.0;
            aperture = 0.1;
        }
        2 => {
            objs = two_spheres();
            lookfrom = Point3::form(13.0,2.0,3.0);
            lookat = Point3::form(0.0,0.0,0.0);
            background= Color::form(0.70, 0.80, 1.00);
            vfov = 20.0;
        }
        3 =>{
            objs = two_perlin_spheres();
            lookfrom = Point3::form(13.0,2.0,3.0);
            lookat = Point3::form(0.0,0.0,0.0);
            background= Color::form(0.70, 0.80, 1.00);
            vfov = 20.0;
        }
        5 =>{
            background= Color::form(0.0, 0.0, 0.0);
            objs = simple_light();
            samples_per_pixel = 400;
            lookfrom = Point3::form(26.0,3.0,6.0);
            lookat = Point3::form(0.0,2.0,0.0);
            vfov = 20.0;
        }
        6 =>{
            objs = cornell_box();
            aspect_ratio = 1.0;
            image_width = 800;
            samples_per_pixel = 100;
            background = Color::form(0.0,0.0,0.0);
            lookfrom = Point3::form(278.0, 278.0, -800.0);
            lookat = Point3::form(278.0, 278.0, 0.0);
            vfov = 40.0;
        }
        _ => {}
    }
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let max_depth = 100;
    let dist_to_focus = 10.0;
    //Camera
    let camera_arc = Arc::new(Camera::new(
        lookfrom, lookat, Vec3::form(0.0,1.0,0.0), vfov, aspect_ratio, aperture, dist_to_focus));


    //World
    let mut world = HittableList::new();
    //读取stl模型三角面
    // let mut stl = StlReader::new_stl_reader("cat.stl".to_string());

    // let x= Arc::new(Metal::form(0.949,0.7529,0.3372,0.3));
    // stl.raed_all_shape_info(&mut objs,x,300.0);
    let bvh_node = BvhNode::form(objs.as_mut_slice(),0.0001,f64::MAX);
    println!("bvh树深度 {}",f32::log2(100.0));
    world.add(Arc::new(bvh_node.unwrap()));
    let world_arc = Arc::new(world);
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
                        pixel_color += ray_color(ray,&background,world_t.borrow(),max_depth);
                    }
                    write_color(file.borrow_mut(), pixel_color,samples_per_pixel);
                }
            }
            let _ = chan.send(());
        });
    }
    for _ in 0..count {
        let _ = rx.recv();
    }
    println!("time cost: {:?} ms",start.elapsed().as_millis());
}
