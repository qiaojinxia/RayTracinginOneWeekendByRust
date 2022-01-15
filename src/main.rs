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
mod macros;
mod pdf;
mod windows;

use std::fmt::{Display, Formatter};
use crate::vec3::Vec3;
use std::{ thread};
use crate::draw::write_color;
use std::borrow::{BorrowMut, Borrow};
use crate::ray::{Point3, Ray};
use crate::hit::{HitRecorder};
use std::sync::{Arc, mpsc, Mutex, RwLock};
use crate::camera::Camera;
use crate::common::{rand_f64, clamp, rand_range_f64};
use std::time::Instant;
use crate::sences::{two_spheres, random_scene, two_perlin_spheres, simple_light, cornell_box, SencesManger};
use std::f64::consts::PI;
use std::fs::File;
use std::io::Write;
use core::time;

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


fn ray_color(ray:Ray,background:&Color,sences_manager:Arc<SencesManger>,depth:i32) -> Color{
    let mut rec = HitRecorder::new();
    if sences_manager.hit(ray, 0.0001, f64::MAX, rec.borrow_mut()){
        let ray = rec.material.clone().unwrap().scatter(&ray, &mut rec);
        let emitted= rec.material.clone().unwrap().clone().emitted(rec.u,rec.v,rec.p.unwrap());
        //蒙特卡洛积分
        let pdf =  0.5 / PI;
        return match ray {
            Some(scattered) => {
                let mut l_in_dir  = Color::new();
                let attenuation = rec.material.clone().unwrap().get_color(&rec);
                let material = rec.material.clone().unwrap();
                let cos_theta = f64::max(Vec3::dot(rec.normal.unwrap() ,ray.unwrap().direction()),0.0);
                match sences_manager.light() {
                    None => {}
                    Some(light) => {
                        let light_p = light.random_sample();
                        let to_light = light_p - rec.p.unwrap();
                        let ray_light = Ray::form(rec.p.unwrap(),to_light);
                        let mut light_rec = HitRecorder::new();
                        let pdf_light = light.pdf_value(light_rec.borrow_mut(),rec.p.unwrap(),to_light);
                        if pdf_light > 0.00001{
                            let brdf = material.
                                scattering_pdf(ray.unwrap().borrow(),rec.borrow(),ray_light.borrow());
                            l_in_dir =  attenuation * brdf * ray_color(ray_light, background,sences_manager.clone(), 0) * cos_theta / pdf_light;
                        }
                    }
                }
                if depth <= 0 {
                    return l_in_dir;
                }
                if rand_range_f64(0.0,1.0) > 0.8 {
                    return l_in_dir;
                }
                let l_dir;
                //对间接光照进行采样 进行积分
                if rec.is_specular {
                    l_dir =  attenuation *
                        ray_color(scattered, background,sences_manager, depth - 1);
                }else{
                    l_dir = emitted + attenuation * material.
                        scattering_pdf(ray.unwrap().borrow(),rec.borrow(),scattered.borrow()) *
                        ray_color(scattered, background,sences_manager, depth - 1) * cos_theta / pdf / 0.8;
                }

                l_in_dir + l_dir
            }
            None => {
                emitted
            }
        }
    }
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
    let mut background= color3!(0,0,0);

    //Image
    let mut image_width = 400;
    let mut samples_per_pixel = 1;
    let mut aspect_ratio = 16.0 / 9.0;

    let mut sences_manager = SencesManger::new();

    match seneces {
        1 => {
            objs = random_scene();
            lookfrom = point3!(13,2,3);
            lookat = point3!(0,0,0);
            background= color3!(0.7, 0.8, 1);
            vfov = 20.0;
            aperture = 0.1;
        }
        2 => {
            objs = two_spheres();
            lookfrom = point3!(13,2,3);
            lookat = point3!(0,0,0);
            background= color3!(0.7, 0.8, 1);
            vfov = 20.0;
        }
        3 =>{
            objs = two_perlin_spheres();
            lookfrom = point3!(13,2,3);
            lookat = point3!(0,0,0);
            background= color3!(0.7, 0, 1);
            vfov = 20.0;
        }
        5 =>{
            background= color3!(0, 0, 0);
            objs = simple_light();
            samples_per_pixel = 400;
            lookfrom = point3!(26,3,6);
            lookat = point3!(0,2,0);
            vfov = 20.0;
        }
        6 =>{
            // objs = cornell_box();
            sences_manager = cornell_box();
            aspect_ratio = 1.0;
            image_width = 500;
            samples_per_pixel = 100;
            background = point3!(0,0,0);
            lookfrom =  point3!(278, 278, -800);
            lookat =  point3!(278, 278, 0);
            vfov = 40.0;
        }
        _ =>  {}
    }
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let max_depth = 100;
    let dist_to_focus = 10.0;
    let collected_iterator: Vec<i32> = (0..image_width*image_height).collect();
    let mut pixel_buffer = Arc::new(RwLock::new(vec![Color::new();image_width as usize * image_height as usize]));
    let mut render_times = Arc::new(RwLock::new(1));
    //Camera
    let camera_arc = Arc::new(Camera::new(
        lookfrom, lookat, Vec3::form(0.0,1.0,0.0), vfov, aspect_ratio, aperture, dist_to_focus));

    //World
    //读取stl模型三角面
    // let mut stl = StlReader::new_stl_reader("cat.stl".to_string());
    // let x= Arc::new(Metal::form(0.949,0.7529,0.3372,0.3));
    // stl.raed_all_shape_info(&mut objs,x,300.0);
    let count = 1; //图形渲染线程数
    let (tx, rx) = mpsc::channel();
    for thread_n in  0 .. count{
        let camera_t = camera_arc.clone();
        let sences_t = sences_manager.clone();
        let arc_pixel_buffer = pixel_buffer.clone();
        let s_chan = tx.clone();
        let global_render_times = render_times.clone();
        let _t = thread::spawn(move ||{
            let mut file = std::fs::File::create(format!("file{}",thread_n)).expect("create failed");
            let per_num = (image_height as f32 / count as f32).ceil() as i32;
            let render_segment_start = per_num * thread_n;
            let mut render_segment_end =  per_num * (thread_n + 1);
            if render_segment_end > image_height{
                render_segment_end = image_height
            }
        for count in  0 .. samples_per_pixel{
            loop{
                if count < *global_render_times.read().unwrap(){
                    break
                }
                thread::sleep(time::Duration::from_millis(10));
            }
            let mut tmp_pixel_buffer = vec![];
            for row in render_segment_start .. render_segment_end{
                for col in 0.. image_width {
                    //往一个像素 偏移非常小的dw方向上 发射不同的光 采样
                        let u = (col as f64 + rand_f64()) / (image_width -1) as f64;
                        let v = (((image_height - 1) - row)  as f64 + rand_f64()) / (image_height - 1) as f64 ;
                        let ray = camera_t.get_ray(u,v);
                        let mut pixel_color = ray_color(ray,&background,sences_t.clone(),max_depth);
                        let mut pixel_buffer = arc_pixel_buffer.read().unwrap();
                        let pixel_index = row * image_width  + col;
                        let sum_pixel = pixel_buffer[pixel_index as usize] + pixel_color;
                        tmp_pixel_buffer.push(sum_pixel);
                    }
                }
                let mut c = 0;
                let mut pixel_buffer = arc_pixel_buffer.write().unwrap();
                for pixel in  tmp_pixel_buffer{
                    pixel_buffer[(render_segment_start * image_width + c) as usize ] = pixel;
                    c += 1;
                }
                s_chan.send(());
            }
        });
    }
    loop{
        for _ in 0..count{
            rx.recv();
        }
        let reader = pixel_buffer.write().unwrap();
        let mut output: File = File::create("image.ppm").unwrap();
        let mut render_times = render_times.write().unwrap();
        output.write("P3 \n500 500 \n255\n".as_ref());
        for c in reader.iter(){
            let d_pixel = write_color(*c,*render_times);
            let s = format!("{}",d_pixel);
            output.write(s.as_bytes());
        }
        *render_times += 1;

    }
    // window.run_loop(windows::MyWindowHandler{ Buffer:pixel_buffer.clone(), TX: rx });
    println!("time cost: {:?} ms",start.elapsed().as_millis());
}
