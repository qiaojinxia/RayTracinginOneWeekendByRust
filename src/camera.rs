use crate::ray::{Point3, Ray};
use crate::vec3::Vec3;
use crate::common::degrees_to_radians;

#[derive(Copy, Clone,Debug)]
pub(crate) struct Camera {
    origin:Point3,
    lower_left_corner:Point3,
    horizontal:Vec3,
    vertical:Vec3,
    lens_radius:f64,
    u:Vec3, v:Vec3, w:Vec3,
}

impl Camera{
    pub(crate) fn new(
        lookfrom:Point3,
        lookat:Point3,
        vup:Vec3,
        vfov:f64,
        aspect_ratio:f64,
        aperture:f64,
        focus_dist:f64) -> Self{
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;


        let w = Vec3::unit_vector(lookfrom - lookat);
        let u = Vec3::unit_vector(Vec3::cross(vup,w));
        let v = Vec3::cross(w,u);



        let origin = lookfrom;
        let horizontal = u * (viewport_width * focus_dist);
        let vertical = v * (viewport_height * focus_dist);
        let lower_left_corner = origin - horizontal / 2.0 -
            vertical / 2.0 - w * focus_dist;
        Self{
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            lens_radius: aperture / 2.0,
            u,
            v,
            w
        }
    }
    pub(crate) fn get_ray(&self,u:f64,v:f64) -> Ray{
        let rd =  Vec3::random_in_unit_disk() * self.lens_radius ;
        let offset = u * rd.x + v * rd.y;
        return Ray::form(self.origin + offset,self.lower_left_corner +
            self.horizontal * u + self.vertical * v - self.origin - offset)
    }
}