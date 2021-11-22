use std::sync::Arc;
use crate::hit::Hittable;
use crate::common::{rand_f64, rand_range_f64};
use crate::ray::Point3;
use crate::Color;
use crate::shape::{Sphere, XyRect, YzRect, XzRect, MBox};
use crate::material::{Lambertian, Dielectric, Metal, Materials, DiffuseLight};
use crate::texture::{CheckerTexture, NoiseTexture};

pub(crate) fn random_scene() -> Vec<Arc<dyn Hittable>> {
    let mut objs:Vec<Arc<dyn Hittable>> = vec![];
    let ground_material = Arc::new(Lambertian::form_color(0.5,0.5,0.5));
    objs.push(Arc::new(Sphere::form(Point3::set(0.0, -1000.0, 0.0), 1000.0, ground_material)));
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
                    sphere_material = Arc::new(Lambertian::form_color(albedo.x,albedo.y,albedo.z));
                    objs.push(Arc::from(Sphere::form(center, 0.2, sphere_material)));
                }else if choose_mat < 0.95{
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = rand_range_f64(0.0,0.5);
                    sphere_material = Arc::new(Metal::form_c(albedo,fuzz));
                    objs.push(Arc::new(Sphere::form(center, 0.2, sphere_material)));
                }else{
                    let sphere_material = Arc::new(Dielectric::form(1.5));
                    objs.push(Arc::new(Sphere::form(center, 0.2, sphere_material)));
                }
            }
        }
    }
    let material1 = Arc::new(Dielectric::form(  1.5));
    objs.push(Arc::new(Sphere::form(Point3::form(0.0, 1.0, 0.0), 1.0, material1)));

    let material2 = Arc::new(Lambertian::form_color(  0.4, 0.2, 0.1));
    objs.push(Arc::new(Sphere::form(Point3::form(-4.0, 1.0, 0.0), 1.0, material2)));


    let material3 = Arc::new(Metal::form(  0.7, 0.6, 0.5, 0.0));
    objs.push(Arc::new(Sphere::form(Point3::form(4.0, 1.0, 0.0), 1.0, material3)));


    objs
}

pub(crate) fn two_spheres() -> Vec<Arc<dyn Hittable>> {
    let mut objs:Vec<Arc<dyn Hittable>> = vec![];
    let checker = Arc::new(CheckerTexture::form_color(Color::form(0.2, 0.3, 0.1), Color::form(0.9, 0.9, 0.9)));
    objs.push(Arc::new(Sphere::form(Point3::form(0.0,-10.0, 0.0), 10.0, Arc::new(Lambertian::form(checker.clone())))));
    objs.push(Arc::new(Sphere::form(Point3::form(0.0,10.0, 0.0), 10.0, Arc::new(Lambertian::form(checker.clone())))));
    objs
}


pub(crate) fn two_perlin_spheres() -> Vec<Arc<dyn Hittable>> {
    let mut objs:Vec<Arc<dyn Hittable>> = vec![];
    let pertext = Arc::new(NoiseTexture::form(4.0));
    objs.push(Arc::new(Sphere::form(Point3::form(0.0,-1000.0,0.0), 1000.0, Arc::new(Lambertian::form(pertext.clone())))));
    objs.push(Arc::new(Sphere::form(Point3::form(0.0, 2.0, 0.0), 2.0, Arc::new(Lambertian::form(pertext.clone())))));
    objs
}


pub(crate) fn simple_light() -> Vec<Arc<dyn Hittable>>{
    let mut objs:Vec<Arc<dyn Hittable>> = vec![];
    let pertext = Arc::new(NoiseTexture::form(4.0));
    objs.push(Arc::new(Sphere::form(Point3::form(0.0,-1000.0,0.0), 1000.0, Arc::new(Lambertian::form(pertext.clone())))));
    objs.push(Arc::new(Sphere::form(Point3::form(0.0,2.0,0.0), 2.0, Arc::new(Lambertian::form(pertext.clone())))));
    let difflight = Arc::new(DiffuseLight::form(Color::form(4.0,4.0,4.0)));

    objs.push(Arc::new(XyRect::form(3.0, 5.0, 1.0, 3.0, -2.0, difflight)));
    objs

}

pub(crate) fn cornell_box() -> Vec<Arc<dyn Hittable>>{
    let mut objs:Vec<Arc<dyn Hittable>> = vec![];
    let red   = Arc::new(Lambertian::form_color(0.65, 0.05, 0.05));
    let white = Arc::new(Lambertian::form_color(0.73, 0.73, 0.73));
    let green = Arc::new(Lambertian::form_color(0.12, 0.45, 0.15));
    let light = Arc::new(DiffuseLight::form(Color::form(15.0, 15.0, 15.0)));

    objs.push(Arc::new(YzRect::form(0.0, 555.0, 0.0, 555.0, 555.0, green.clone())));
    objs.push(Arc::new(YzRect::form(0.0, 555.0, 0.0, 555.0, 0.0, red.clone())));
    objs.push(Arc::new(XzRect::form(213.0, 343.0, 227.0, 332.0, 554.0, light)));
    objs.push(Arc::new(XzRect::form(0.0, 555.0, 0.0, 555.0, 0.0, white.clone())));
    objs.push(Arc::new(XzRect::form(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())));
    objs.push(Arc::new(XyRect::form(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())));

    objs.push(Arc::new(MBox::form(Point3::form(130.0, 0.0, 65.0), Point3::form(295.0, 165.0, 230.0), white.clone())));
    objs.push(Arc::new(MBox::form(Point3::form(265.0, 0.0, 295.0), Point3::form(430.0, 330.0, 460.0), white.clone())));


    objs
}


pub(crate) fn sences1() -> Vec<Arc<dyn Hittable>>{
    // //Materials
    // let m_ground = Arc::new(Lambertian::form(Arc::new(CheckerTexture::form_color(Color::form(0.2, 0.3, 0.1),Color::form(0.9, 0.9, 0.9)))));
    // // let m_center = Arc::new(Lambertian::form(0.7,0.3,0.3));
    // // let m_left= Arc::new(Metal::form(0.8,0.8,0.8,0.3));
    // let m_center = Arc::new(Dielectric::form(1.5));
    // let m_left= Arc::new(Dielectric::form(1.5));
    // let m_left1= Arc::new(Dielectric::form(0.8));
    // let m_right= Arc::new(Metal::form(0.8,0.6,0.2,1.0));
    // let golden= Arc::new(Metal::form(0.1,1.0,0.0,1.0));

    // let mut objs:Vec<Arc<dyn Hittable>> = vec![];

    // world.add(Arc::new(Sphere::form(Point3::form(0.0,-100.5,-1.0),100.0,m_ground)));
    // world.add(Arc::new(Sphere::form(Point3::form(0.0,0.0,-1.0),0.5,m_center)));
    // world.add(Arc::new(Sphere::form(Point3::form(-1.0,0.0,-1.0),0.5,m_left)));
    // objs.push(Arc::new(Sphere::form(Point3::form(5.0,0.0,5.0),-10.0,m_left1.clone())));
    // objs.push(Arc::new(Sphere::form(Point3::form(1.0,0.0,-1.0),10.0,m_right.clone())));
    // world.add(Arc::new(Triangle::form_x(Point3::form(0.0,1.0,-1.0),1.0,1.0,x)));
    vec![]

}