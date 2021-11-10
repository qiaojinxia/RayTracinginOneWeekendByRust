use crate::ray::{Point3, Ray};
use crate::vec3::Vec3;
use crate::common::degrees_to_radians;

#[derive(Copy, Clone,Debug)]
pub(crate) struct Camera {
    lookfrom:Point3,
    lookat:Point3,
    vup:Vec3,
    origin:Point3,
    lower_left_corner:Point3,
    horizontal:Vec3,
    vertical:Vec3,
}

impl Camera{
    pub(crate) fn new(lookfrom:Point3,lookat:Point3,vup:Vec3,vfov:f64, aspect_ratio:f64) -> Self{
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let w = Vec3::unit_vector(lookfrom - lookat);
        let u = Vec3::unit_vector(Vec3::cross(vup,w));
        let v = Vec3::cross(w,u);

        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        let origin = lookfrom;
        let horizontal = u * viewport_width;
        let vertical = v * viewport_height;
        let lower_left_corner = origin - horizontal / 2.0 -
            vertical / 2.0 - w;
        Self{
            lookfrom: Vec3::new(),
            lookat: Vec3::new(),
            vup: Vec3::new(),
            origin,
            lower_left_corner,
            horizontal,
            vertical
        }
    }
    pub(crate) fn get_ray(&self,u:f64,v:f64) -> Ray{
        return Ray::form(self.origin,self.lower_left_corner +
            self.horizontal * u + self.vertical * v - self.origin)
    }
}