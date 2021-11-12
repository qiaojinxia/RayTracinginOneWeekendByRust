use crate::ray::{Point3, Ray};
use crate::vec3::Vec3;
use crate::hit::{Hittable, HitRecorder};
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use crate::material::Materials;
use std::f64::MIN;
use crate::common;

pub(crate) struct Sphere{
    center:Point3,
    radius:f64,
    pub(crate) material:Option<Arc<dyn Materials>>,
}

impl Sphere {
    pub(crate) fn form(center:Point3, radius:f64, material: Arc<dyn Materials>) -> Self{
        Self{
            center,
            radius,
            material:Some(material)
        }
    }
}

impl Debug for Sphere {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:?}",self)
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
        rec.material = self.material.clone();
        let outward_normal = (rec.p.unwrap() - self.center) / self.radius;
        rec.set_face_normal(ray,outward_normal);
        return true;
    }
}


pub(crate) struct Triangle {
    pub(crate) p1:Point3,
    pub(crate) p2:Point3,
    pub(crate) p3:Point3,
    pub(crate) material:Option<Arc<dyn Materials>>,
}

impl Triangle{
    pub(crate) fn form(p1:Point3, p2:Point3, p3: Point3,material:Arc<dyn Materials>) -> Self{
        Self{
            p1,
            p2,
            p3,
            material: Some(material)
        }
    }
    pub(crate) fn form_x(center:Point3, height:f64,weight:f64,material:Arc<dyn Materials>) -> Self{
        let p1 = Point3::form( center.x,center.y + height / 2.0,center.z);
        let p2 = Point3::form(center.x - weight/2.0,center.y - height / 2.0,center.z);
        let p3 = Point3::form(center.x + weight/2.0,center.y - height / 2.0,center.z);
        Self{
            p1,
            p2,
            p3,
            material: Some(material)
        }
    }

}

impl Debug for Triangle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

//三角形的重心公式 然后使用 克莱姆法则求解
impl Hittable for Triangle{
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, rec: &mut HitRecorder) -> bool {
        let e1 = self.p2 - self.p1;
        let e2 = self.p3 - self.p1;
        let t1 = ray.origin() - self.p1;
        let p1 = Vec3::cross(ray.direction(),e2);
        let p2 = Vec3::cross(t1,e1 );
        let mut molecule = Vec3::dot(p1,t1);
        let denominator = Vec3::dot(p1,e1);
        if denominator > - common::MIN && denominator <  common::MIN{
            return false
        }
        let u = molecule / denominator;
        if u < 0.0 || u > 1.0{
            return false;
        }
        molecule = Vec3::dot(p2,ray.direction());
        let v = molecule / denominator;
        if v < 0.0 || u + v > 1.0{
            return false;
        }

        molecule = Vec3::dot(p2,e2);
        let t = molecule / denominator;
        if t < t_min || t_max < t{
            return false;
        }
        rec.material = self.material.clone();
        rec.t = t;
        //计算 三角行的 法线 2条边求叉积
        let mut outward_normal = Vec3::cross(e1,e2).unit_vector();
        rec.set_face_normal(ray,outward_normal);
        rec.p = Some(ray.at(rec.t));
        rec.front_face = true;
        true

    }
}
