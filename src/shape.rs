use crate::ray::{Point3, Ray};
use crate::vec3::Vec3;
use crate::hit::{Hittable, HitRecorder};
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use crate::material::Materials;
use crate::common::{cmp_f64, f64_near_zero, Axis, Tuple, degrees_to_radians};
use std::f64::consts::PI;
use crate::{point3};

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
    pub(crate) fn get_sphere_uv(p:Point3) -> Tuple{
        let theta = (-p.y).acos();
        let phi = (-p.z).atan2(p.x) + PI;
        let u = phi / (2.0 * PI);
        let v = theta / PI;
        Tuple::UV(u,v)
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
        let uv =  Self::get_sphere_uv(outward_normal);
        match uv {
            Tuple::UV(u, v) => {
                rec.u = u;
                rec.v = v;
            }
        }
        rec.set_face_normal(ray,outward_normal);
        return true;
    }

    fn bounding_box(&self) -> Option<AABB> {
        let r = point3!(self.radius,self.radius,self.radius);
            Some(AABB::form(
                self.center - r,
                self.center + r,
            ))
    }

    fn get_center_point(&self, a: &Axis) -> f64 {
        a.call(self.center)
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
        let p1 = point3!( center.x,center.y + height / 2.0,center.z);
        let p2 = point3!(center.x - weight/2.0,center.y - height / 2.0,center.z);
        let p3 = point3!(center.x + weight/2.0,center.y - height / 2.0,center.z);
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
    fn bounding_box(&self) -> Option<AABB> {
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

    fn get_center_point(&self, a: &Axis) -> f64 {
        a.call(self.w)
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
        let inv_d = point3!(1.0 / ray.direction().x,1.0 / ray.direction().y,1.0 / ray.direction().z);
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

pub(crate) struct XyRect{
    x0:f64,
    x1:f64,
    y0:f64,
    y1:f64,
    k:f64,
    mp:Option<Arc<dyn Materials>>,
}

impl Debug for XyRect {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl XyRect{
    pub(crate) fn form(x0:f64,x1:f64,y0:f64,y1:f64,k:f64,ma:Arc<dyn Materials>) -> Self{
        Self{
            x0:x0.min(x1),
            x1:x0.max(x1),
            y0:y0.min(y1),
            y1:y0.max(y1),
            k,
            mp: Some(ma),
        }
    }
}
impl Hittable for XyRect{
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, rec: &mut HitRecorder) -> bool {
        let t = (self.k - ray.origin().z) / ray.direction().z;
        if t < t_min || t > t_max{
            return false;
        }
        let x = ray.origin().x + t * ray.direction().x;
        let y = ray.origin().y + t * ray.direction().y;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1{
            return false;
        }
        rec.u = (x-self.x0) / (self.x1-self.x0);
        rec.v = (y-self.y0) / (self.y1-self.y0);
        rec.t = t;
        let outward_normal = Vec3::form(0.0, 0.0, 1.0);
        rec.set_face_normal(ray, outward_normal);
        rec.material = Some(self.mp.clone().unwrap());
        rec.p = Some(ray.at(t));
        return true;
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB::form(point3!(self.x0,self.y0, self.k-0.0001),
                        point3!(self.x1, self.y1, self.k+0.0001)))
    }

    fn get_center_point(&self, a: &Axis) -> f64 {
       match a {
           Axis::X => { self.x0 + (self.x1 - self.x0) / 2.0}
           Axis::Y => { self.y0 + (self.y1 - self.y0) / 2.0 }
           Axis::Z => { self.k }
       }
    }
}


pub(crate) struct XzRect{
    x0:f64,
    x1:f64,
    z0:f64,
    z1:f64,
    k:f64,
    mp:Option<Arc<dyn Materials>>,
}

impl XzRect{
    pub(crate) fn form(x0:f64,x1:f64,z0:f64,z1:f64,k:f64,ma:Arc<dyn Materials>) -> Self{
        Self{
            x0:x0.min(x1),
            x1:x0.max(x1),
            z0:z0.min(z1),
            z1:z0.max(z1),
            k,
            mp: Some(ma),
        }
    }
}

impl Debug for XzRect {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Hittable for XzRect{
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, rec: &mut HitRecorder) -> bool {
        let t = (self.k - ray.origin().y) / ray.direction().y;
        if t < t_min || t > t_max{
            return false;
        }
        let x = ray.origin().x + t * ray.direction().x;
        let z = ray.origin().z + t * ray.direction().z;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1{
            return false;
        }
        rec.u = (x-self.x0) / (self.x1-self.x0);
        rec.v = (z-self.z0) / (self.z1-self.z0);
        rec.t = t;
        let outward_normal = Vec3::form(0.0, 1.0, 0.0);
        rec.set_face_normal(ray, outward_normal);
        rec.material = Some(self.mp.clone().unwrap());
        rec.p = Some(ray.at(t));
        return true;
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB::form(point3!(self.x0, self.k - 0.1001, self.z0),
                        point3!(self.x1, self.k + 0.1001, self.z1)))
    }

    fn get_center_point(&self, a: &Axis) -> f64 {
        match a {
            Axis::X => { self.x0 + (self.x1 - self.x0) / 2.0}
            Axis::Y => { self.k }
            Axis::Z => { self.z0 + (self.z1 - self.z0) / 2.0 }
        }
    }
}


pub(crate) struct YzRect{
    y0:f64,
    y1:f64,
    z0:f64,
    z1:f64,
    k:f64,
    mp:Option<Arc<dyn Materials>>,
}


impl YzRect{
    pub(crate) fn form(y0:f64,y1:f64,z0:f64,z1:f64,k:f64,ma:Arc<dyn Materials>) -> Self{
        Self{
            y0:y0.min(y1),
            y1:y0.max(y1),
            z0:z0.min(z1),
            z1:z0.max(z1),
            k,
            mp: Some(ma),
        }
    }
}

impl Debug for YzRect {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Hittable for YzRect{
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, rec: &mut HitRecorder) -> bool {
        let t = (self.k - ray.origin().x) / ray.direction().x;
        if t < t_min || t > t_max{
            return false;
        }
        let y = ray.origin().y + t * ray.direction().y;
        let z = ray.origin().z + t * ray.direction().z;
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1{
            return false;
        }
        rec.u = (y-self.y0) / (self.y1-self.y0);
        rec.v = (z-self.z0) / (self.z1-self.z0);
        rec.t = t;
        let outward_normal = Vec3::form(1.0, 0.0, 0.0);
        rec.set_face_normal(ray, outward_normal);
        rec.material = Some(self.mp.clone().unwrap());
        rec.p = Some(ray.at(t));
        return true;
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB::form(point3!(self.k - 0.0001, self.y0, self.z0),
                        point3!(self.k + 0.0001, self.y1, self.z1)))
    }

    fn get_center_point(&self, a: &Axis) -> f64 {
        match a {
            Axis::X => { self.k }
            Axis::Y => { self.y0 + (self.y1 - self.y0) / 2.0 }
            Axis::Z => { self.z0 + (self.z1 - self.z0) / 2.0 }
        }
    }
}

pub(crate) struct MBox{
    box_min:Point3,
    box_max:Point3,
    sides:Vec<Arc<dyn Hittable>>
}

impl Debug for MBox {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl MBox{
    pub(crate) fn form(p0:Point3,p1:Point3,ma:Arc<dyn Materials>) -> Self{
        if p0.x > p1.x || p0.y > p1.y || p0.z > p1.z {
            panic!("定义点的顺序错误,应从小开始!")
        }
        let mut hittable_list:Vec<Arc<dyn Hittable>> = vec![];
        hittable_list.push(Arc::new(XyRect::form(p0.x, p1.x, p0.y, p1.y, p1.z, ma.clone())));
        hittable_list.push(Arc::new(XyRect::form(p0.x, p1.x, p0.y, p1.y, p0.z, ma.clone())));

        hittable_list.push(Arc::new(XzRect::form(p0.x, p1.x, p0.z, p1.z, p1.y, ma.clone())));
        hittable_list.push(Arc::new(XzRect::form(p0.x, p1.x, p0.z, p1.z, p0.y, ma.clone())));

        hittable_list.push(Arc::new(YzRect::form(p0.y, p1.y, p0.z, p1.z, p1.x, ma.clone())));
        hittable_list.push( Arc::new(YzRect::form(p0.y, p1.y, p0.z, p1.z, p0.x, ma.clone())));
        Self{
            box_min:p0,
            box_max:p1,
            sides:hittable_list,
        }
    }
}

impl Hittable for MBox{
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, rec: &mut HitRecorder) -> bool {
        let mut can_hit = false;
        let mut max_t = t_max;
        for objs in self.sides.iter(){
            if objs.clone().hit(ray,t_min,max_t,rec){
                max_t = rec.t;
                can_hit = true;
            }
        }
        can_hit
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB::form(self.box_min,self.box_max))
    }

    fn get_center_point(&self, a: &Axis) -> f64 {
        let center_point =  self.box_min + (self.box_max  - self.box_min) / 2.0;
            match a {
                Axis::X => { center_point.x }
                Axis::Y => { center_point.y}
                Axis::Z => { center_point.z }
            }
    }
}

pub(crate) struct YRotate{
    obj_ptr:Option<Arc<dyn Hittable>>,
    sin_theta:f64,
    cos_theta:f64,
    has_box:bool,
    aabb:Option<AABB>,
}

impl Debug for YRotate {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}


impl YRotate{
    pub(crate) fn form(p:Arc<dyn Hittable>,angle:f64) -> Self{
        let radians = degrees_to_radians(angle);
        let mut after_rotate_obj = Self{
            obj_ptr: Some(p.clone()),
            sin_theta: radians.sin(),
            cos_theta: radians.cos(),
            has_box:false,
            aabb:None,
        };
        match p.bounding_box(){
            None => { }
            Some(aabb) => {
                let p0 = aabb.minimum;
                let p1 = aabb.maximum;
                let mut points = vec![];

                points.push(point3!(p0.x, p1.y, p0.z));
                points.push(point3!(p0.x, p1.y, p1.z));
                points.push(point3!(p0.x, p0.y, p1.z));

                points.push(point3!(p1.x, p0.y, p1.z));
                points.push(point3!(p1.x, p0.y, p0.z));
                points.push(point3!(p1.x, p1.y, p0.z));

                points.push(p0);
                points.push(p1);

                let mut min = point3!(f64::MAX,f64::MAX,f64::MAX);
                let mut max = point3!(f64::MIN,f64::MIN,f64::MIN);

                for i in points.iter(){
                    min = Vec3::min(min,Vec3::rotate_y(*i,after_rotate_obj.sin_theta,after_rotate_obj.cos_theta));
                    max = Vec3::max(max,Vec3::rotate_y(*i,after_rotate_obj.sin_theta,after_rotate_obj.cos_theta));
                }
                after_rotate_obj.has_box = true;
                after_rotate_obj.aabb = Some(AABB::form(min,max))

            }
        }
        after_rotate_obj
    }
}


impl Hittable for YRotate{
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, rec: &mut HitRecorder) -> bool {
        let origin = ray.origin();
        let direction = ray.direction();
        let new_origin = Vec3::rotate_y(origin,-self.sin_theta,self.cos_theta);
        let new_dir = Vec3::rotate_y(direction,-self.sin_theta,self.cos_theta);
        let rotated_ray = Ray::form (new_origin,new_dir);
        if !self.obj_ptr.clone().unwrap().hit(rotated_ray, t_min, t_max, rec) {
            return false;
        }
        let p = rec.p.unwrap();
        let normal = rec.normal.unwrap();

        let rotated_p = Vec3::rotate_y(p,self.sin_theta,self.cos_theta);
        let rotated_normal = Vec3::rotate_y(normal,self.sin_theta,self.cos_theta);
        rec.p = Some(rotated_p);
        rec.set_face_normal(rotated_ray, rotated_normal);
        true
    }

    fn bounding_box(&self) -> Option<AABB> {
        self.aabb
    }

    fn get_center_point(&self, a: &Axis) -> f64 {
        let center = self.aabb.unwrap().minimum + (self.aabb.unwrap().maximum - self.aabb.unwrap().minimum) / 2.0;
        match a {
            Axis::X => { center.x}
            Axis::Y => { center.y}
            Axis::Z => { center.z}
        }
    }
}

pub(crate) struct Translate{
    obj_ptr:Option<Arc<dyn Hittable>>,
    offset:Vec3,
}

impl Translate{
    pub(crate) fn form(p:Arc<dyn Hittable>,displacement:Vec3) -> Self{
        Self{
            obj_ptr: Some(p.clone()),
            offset: displacement,
        }
    }
}

impl Debug for Translate {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Hittable for Translate{
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, rec: &mut HitRecorder) -> bool {
        let new_origin = ray.origin() - self.offset;
        let moved_ray = Ray::form(new_origin, ray.direction());
        if !self.obj_ptr.clone().unwrap().hit(moved_ray, t_min, t_max, rec){
            return false
        }
        let mut p = rec.p.unwrap();
        p += self.offset;
        rec.p = Some(p);
        rec.set_face_normal(moved_ray, rec.normal.unwrap());
        true
    }

    fn bounding_box(&self) -> Option<AABB> {
        let child_box = self.obj_ptr.clone().unwrap().bounding_box().unwrap();
        Some(AABB::form(
            child_box.minimum + self.offset,
            child_box.maximum + self.offset,
        ))
    }

    fn get_center_point(&self, a: &Axis) -> f64 {
        self.obj_ptr.clone().unwrap().get_center_point(a) + a.call(self.offset)
    }
}