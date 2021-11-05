use crate::ray::{Point3, Ray};
use crate::vec3::Vec3;

pub(crate) fn hit_sphere(center: Point3, radius:f64, ray:Ray) -> bool{
    let oc = ray.origin() - center;
    let a = Vec3::dot(ray.direction(),ray.direction());
    let b = 2.0 * Vec3::dot(oc,ray.direction());
    let c = Vec3::dot(oc,oc) - radius * radius;
    let discriminant = b * b -  4.0 * a  * c;
    discriminant > 0.0
}