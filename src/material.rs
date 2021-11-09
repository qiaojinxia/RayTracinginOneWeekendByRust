use crate::ray::Ray;
use crate::hit::HitRecorder;
use crate::Color;
use crate::vec3::Vec3;
use std::borrow::Borrow;
use std::rc::Rc;
use std::sync::Arc;

pub(crate) trait Materials:Send + Sync{
    fn scatter(&self,ray_in:&Ray,rec:HitRecorder) -> Option<Ray>;
    fn get_color(&self) -> Color;
}

pub(crate) struct Lambertian{
    pub(crate) albedo:Color,
}

impl Lambertian{
    pub(crate) fn form(r:f64,g:f64,b:f64) -> Lambertian{
        Self{
            albedo: Vec3::form(r,g,b),
        }
    }
}

impl Materials for Lambertian{
    fn scatter(&self, _ray_in: &Ray, rec: HitRecorder) -> Option<Ray> {
        let mut scatter_direction = rec.normal.unwrap() + Vec3::random_unit_vector();
        if scatter_direction.near_zero(){
            scatter_direction = rec.normal.unwrap();
        }
        return Some(Ray::form(rec.p.unwrap(), scatter_direction))
    }

    fn get_color(&self) -> Color {
        self.albedo
    }
}

pub(crate) struct Metal{
    albedo:Color,
    fuzz:f64,
}

impl Metal{
    pub(crate) fn form(r:f64, g:f64, b:f64, mut f:f64) -> Metal{
        if f > 1.0{
            f = 1.0
        }
        Self{
            fuzz:f,
            albedo: Color::form(r,g,b),
        }
    }
}
impl Materials for Metal{
    fn scatter(&self, ray_in: &Ray, rec: HitRecorder) -> Option<Ray> {
        let reflected = Vec3::reflect(ray_in.direction().unit_vector(),rec.normal.unwrap());
        let scattered = Ray::form(rec.p.unwrap(), reflected + Vec3::random_in_unit_sphere() * self.fuzz  );
        if Vec3::dot(scattered.direction(),rec.normal.unwrap()) > 0.0{
           return Some(scattered);
        }
        None
    }

    fn get_color(&self) -> Color {
        self.albedo
    }
}