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

type Color = Vec3;

pub(crate) fn random_scene() -> Vec<Arc<dyn Hittable>> {
    let mut world:Vec<Arc<dyn Hittable>> = vec![];
    let ground_material = Arc::new(Lambertian::form(0.5,0.5,0.5));
    world.push(Arc::new(Sphere::form(Point3::set(0.0,-1000.0,0.0),1000.0,ground_material)));
    for i in -11 .. 11{
        for j in -11 .. 11{
            let a = i as f64;
            let b = j as f64;
            let choose_mat = rand_f64();
            let center = Point3::form(a + 0.9 * rand_f64(),0.2,b + 0.9 * rand_f64());
            if (center - Point3::form(4.0,0.2,0.0)).length() > 0.9{
                let sphere_material:Arc<dyn Materials>;
                if choose_mat < 0.8{
                    let albedo = Color::random() * Color::random();
                    sphere_material = Arc::new(Lambertian::form(albedo.x,albedo.y,albedo.z));
                    world.push(Arc::from(Sphere::form(center, 0.2, sphere_material)));
                }else if choose_mat < 0.95{
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = rand_range_f64(0.0,0.5);
                    sphere_material = Arc::new(Metal::form_c(albedo,fuzz));
                    world.push(Arc::new(Sphere::form(center,0.2,sphere_material)));
                }else{
                    let sphere_material = Arc::new(Dielectric::form(1.5));
                    world.push(Arc::new(Sphere::form(center,0.2,sphere_material)));
                }
            }
        }
    }
    let material1 = Arc::new(Dielectric::form(  1.5));
    world.push(Arc::new(Sphere::form(Point3::form(0.0,1.0,0.0),1.0,material1)));

    let material2 = Arc::new(Lambertian::form(  0.4, 0.2, 0.1));
    world.push(Arc::new(Sphere::form(Point3::form(-4.0,1.0,0.0),1.0,material2)));


    let material3 = Arc::new(Metal::form(  0.7, 0.6, 0.5, 0.0));
    world.push(Arc::new(Sphere::form(Point3::form(4.0,1.0,0.0),1.0,material3)));


    world
}

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
    if world.hit(ray, 0.000001, f64::MAX, rec.borrow_mut()){
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
    let max_depth = 100;


    // //Materials
    let m_ground = Arc::new(Lambertian::form(176.0/255.0,196.0/255.0,222.0/255.0,));
    // // let m_center = Arc::new(Lambertian::form(0.7,0.3,0.3));
    // // let m_left= Arc::new(Metal::form(0.8,0.8,0.8,0.3));
    // let m_center = Arc::new(Dielectric::form(1.5));
    // let m_left= Arc::new(Dielectric::form(1.5));
    let m_left1= Arc::new(Dielectric::form(0.8));
    // let m_right= Arc::new(Metal::form(0.8,0.6,0.2,1.0));
    // let golden= Arc::new(Metal::form(0.1,1.0,0.0,1.0));

    let mut objs:Vec<Arc<dyn Hittable>> = vec![];
    //World
    let mut world = HittableList::new();
    world.add(Arc::new(Sphere::form(Point3::form(0.0,-100.5,-1.0),100.0,m_ground)));
    // world.add(Arc::new(Sphere::form(Point3::form(0.0,0.0,-1.0),0.5,m_center)));
    // world.add(Arc::new(Sphere::form(Point3::form(-1.0,0.0,-1.0),0.5,m_left)));
    objs.push(Arc::new(Sphere::form(Point3::form(5.0,0.0,5.0),-10.0,m_left1.clone())));
    // objs.push(Arc::new(Sphere::form(Point3::form(1.0,0.0,-1.0),10.0,m_right.clone())));
    // world.add(Arc::new(Triangle::form_x(Point3::form(0.0,1.0,-1.0),1.0,1.0,x)));

    //读取stl模型三角面
    let mut stl = StlReader::new_stl_reader("cat.stl".to_string());

    let x= Arc::new(Metal::form(0.949,0.7529,0.3372,1.0));
    stl.raed_all_shape_info(&mut objs,x,300.0);
    let bvh_node = BvhNode::form(objs.as_mut_slice(),0.0001,f64::MAX,0);
    println!("bvh树深度 {}",f32::log2(100.0));
    world.add(Arc::new(bvh_node.unwrap()));
    let world_arc = Arc::new(world);
    //Camera
    let lf = Point3::form(0.0,0.0,40.0);
    let la = Vec3::form(0.0,15.0,-5.0);
    let dist_to_focus = (lf-la).length();
    let camera_arc = Arc::new(Camera::new(
        lf, la, Vec3::form(0.0,1.0,0.0),45.0,aspect_ratio,2.0,dist_to_focus));
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
                    write_color(file.borrow_mut(), pixel_color,samples_per_pixel);
                }
            }
            let _ = chan.send(());
        });
    }
    for _ in 0..count {
        let _ = rx.recv();
    }
}
