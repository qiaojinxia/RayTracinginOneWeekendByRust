use crate::ray::Ray;
use crate::hit::HitRecorder;
use crate::Color;
use crate::vec3::Vec3;

pub(crate) trait Materials:Send + Sync{
    fn scatter(self,ray_in:&Ray,rec:HitRecorder,attenuation:Color,scattered:&Ray) -> bool;
}

pub(crate) struct Lambertian{
    pub(crate) albedo:Color,
}

impl Lambertian{
    fn form(a:Color) -> Self{
        Self{
            albedo: a,
        }
    }
}

impl Materials for Lambertian{
    fn scatter(self, ray_in: &Ray, rec: HitRecorder, mut attenuation: Color, mut scattered: &Ray) -> bool {
        let mut scatter_direction = rec.normal.unwrap() + Vec3::random_unit_vector();
        if scatter_direction.near_zero(){
            scatter_direction = rec.normal.unwrap();
        }
        scattered = &Ray::form(rec.p.unwrap(), scatter_direction);
        attenuation = self.albedo;
        return true
    }
}

pub(crate) struct Metal{
    albedo:Color,
}

impl Materials for Metal{
    fn scatter(self, ray_in: &Ray, rec: HitRecorder, mut attenuation: Color, scattered: &mut Ray) -> bool {
        let reflected = Vec3::reflect(ray_in.direction().unit_vector(),rec.normal.unwrap());
        scattered = &Ray::form(rec.p.unwrap(), reflected);
        attenuation = self.albedo;
        return Vec3::dot(scattered.direction(),rec.normal.unwrap()) > 0.0 ;
    }
}