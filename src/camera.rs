use crate::ray::{Point3, Ray};
use crate::vec3::Vec3;

pub(crate) struct Camera {
    origin:Point3,
    lower_left_corner:Point3,
    horizontal:Vec3,
    vertical:Vec3,
}

impl Camera {
    pub(crate) fn new(origin:Point3, viewport_height:f64) -> Self{
        let aspect_ratio = 16.0 / 9.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;
        let origin = origin;
        let horizontal = Vec3::form(viewport_width, 0.0, 0.0);
        let vertical = Vec3::form(0.0,viewport_height,0.0);
        let lower_left_corner = origin - horizontal/2.0 -
            vertical/2.0 - Vec3::form(0.0, 0.0, focal_length);
        Self{
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