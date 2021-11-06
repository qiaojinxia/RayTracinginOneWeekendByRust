use crate::ray::{Point3, Ray};
use crate::vec3::Vec3;
use crate::hit::{Hittable, HitRecorder};

pub(crate) struct Sphere{
    center:Point3,
    radius:f64,
}

impl Sphere {
    pub(crate) fn form(center:Point3,radius:f64) -> Self{
        Self{
            center,
            radius
        }
    }
}

//计算射线是否能击中圆形
impl Hittable for Sphere{
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, rec: &mut HitRecorder) -> bool {
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b =  Vec3::dot(oc,ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b -  a  * c;
        if discriminant < 0.0 {
            return false
        }
        let squared = discriminant.sqrt();
        let mut root = (- half_b - squared) / a;
        if root < t_min || t_max < root{
            root = (- half_b + squared) / a;
            if root < t_min || t_max < root{
                return false;
            }
        }
        rec.t = root;
        rec.p = Some(ray.at(rec.t));
        let outward_normal = (rec.p.unwrap() - self.center) / self.radius;
        rec.set_face_normal(ray,outward_normal);
        return true;
    }
}


pub(crate) struct Love{

}

impl Hittablefor for Love{

}