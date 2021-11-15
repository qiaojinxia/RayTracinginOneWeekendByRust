use crate::ray::{Point3, Ray};
use crate::vec3::Vec3;
use crate::hit::{Hittable, HitRecorder};
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use crate::material::Materials;
use crate::common::{cmp_f64, f64_near_zero};


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
        write!(f,"{:?}",self.center)
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

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        let r = Point3::form(self.radius,self.radius,self.radius);
            Some(AABB::form(
                self.center - r,
                self.center + r,
            ))
    }

    fn get_center_point(&self, dir: i32) -> f64 {
        if dir == 0 {
            return self.center.x
        }else if dir == 1{
            return self.center.y
        }else if dir == 2{
            return self.center.z
        }
        panic!("错误的索引")
    }
}


pub(crate) struct Triangle {
    pub(crate) p1:Point3,
    pub(crate) p2:Point3,
    pub(crate) p3:Point3,
    pub(crate) w:Point3,
    pub(crate) material:Option<Arc<dyn Materials>>,
}

impl Triangle{
    pub(crate) fn form(p1:Point3, p2:Point3, p3: Point3,material:Arc<dyn Materials>) -> Self{
        Self{
            p1,
            p2,
            p3,
            w:(p1+p2+p3) / 3.0,
            material: Some(material)
        }
    }
    pub(crate) fn form_by_center(center:Point3, height:f64,weight:f64,material:Arc<dyn Materials>) -> Self{
        let p1 = Point3::form( center.x,center.y + height / 2.0,center.z);
        let p2 = Point3::form(center.x - weight/2.0,center.y - height / 2.0,center.z);
        let p3 = Point3::form(center.x + weight/2.0,center.y - height / 2.0,center.z);
        Self{
            p1,
            p2,
            p3,
            w:(p1+p2+p3) / 3.0,
            material: Some(material)
        }
    }

}

impl Debug for Triangle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.w)
    }
}

//三角形的重心公式 然后使用 克莱姆法则求解
impl Hittable for Triangle{
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, rec: &mut HitRecorder) -> bool {
        let e1 = self.p2 - self.p1;
        let e2 = self.p3 - self.p1;
        let t1 = ray.origin() - self.p1 ;
        let p1 = Vec3::cross(ray.direction(),e2);
        let p2 = Vec3::cross(t1,e1 );
        let mut molecule = Vec3::dot(p1,t1);
        let denominator = Vec3::dot(p1,e1);
        if f64_near_zero(denominator) {
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
        //三角法向量 = 2条边求叉积
        let outward_normal = Vec3::cross(e1,e2).unit_vector();
        rec.set_face_normal(ray,outward_normal);
        rec.p = Some(ray.at(rec.t));
        true

    }
    //计算三角面的 包围盒 求出 最小的 三个点 和最大三个点 构成的长方体
    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        let mut min_point = Point3::new();
        let mut max_point = Point3::new();
        for i in 0..3{
            let i1 = self.p1.get_field(i);
            let i2 = self.p2.get_field(i);
            let i3 = self.p3.get_field(i);
            let mut nums = vec![i1, i2, i3];
            nums.sort_by(|a, b| cmp_f64(*a, *b));
            min_point.set_i_field(i,nums[0]);
            max_point.set_i_field(i,nums.pop().unwrap());
        }
        Some(AABB::form(min_point,max_point))
    }

    fn get_center_point(&self, dir: i32) -> f64 {
        if dir == 0{
            return self.w.x;
        }else if dir == 1{
            return self.w.y;
        }else if dir == 2{
            return self.w.z;
        }
        panic!("错误的索引!")
    }
}

#[derive(Copy, Clone)]
pub(crate) struct AABB{
    pub(crate) minimum:Vec3,
    pub(crate) maximum:Vec3,
}

impl AABB{
    pub(crate) fn form(a:Vec3,b:Vec3) -> Self{
        Self{
            minimum:a,
            maximum:b
        }
    }
}

impl Debug for AABB {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:?}",self)
    }
}

impl AABB{
    pub(crate) fn hit(&self, ray: Ray, t_min: f64, t_max: f64, _rec: &mut HitRecorder) -> bool {
        let inv_d = Point3::form(1.0 / ray.direction().x,1.0 / ray.direction().y,1.0 / ray.direction().z);
        let t_in = (self.minimum - ray.origin()) * inv_d;
        let t_out=(self.maximum - ray.origin()) * inv_d;
        let min_t = Vec3::min(t_in,t_out);
        let max_t = Vec3::max(t_in,t_out);
        //求最晚进入的时间(3对面都进入) 和 最早一条线离开的时间(离开一个对面就算离开包围盒)
        let t0 = f64::max(f64::max(min_t.x,f64::max(min_t.y,min_t.z)),t_min);
        let t1 = f64::min(f64::min(max_t.x,f64::min(max_t.y,max_t.z)),t_max);
        if t0 > t1{
            return false
        }
        return true;
    }
}


