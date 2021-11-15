use crate::ray::{Point3, Ray};
use crate::vec3::Vec3;
use std::fmt::{Debug};
use std::sync::Arc;
use crate::material::Materials;
use crate::shape::AABB;
use crate::common::Axis;


#[derive( Clone)]
pub(crate) struct HitRecorder {
    pub(crate) p:Option<Point3>,
    pub(crate) normal:Option<Vec3>,
    pub(crate) material:Option<Arc<dyn Materials>>,
    pub(crate) t:f64,
    pub(crate) front_face:bool,
}

pub(crate) trait Hittable:Send+ Sync +Debug  {
    fn hit(&self,ray:Ray,t_min:f64,t_max:f64,rec:&mut HitRecorder) -> bool;
    fn bounding_box(&self) -> Option<AABB>;
    fn get_center_point(&self, a:&Axis) -> f64;
}

//计算射线物体的前面还是后面
impl HitRecorder{

    pub(crate) fn new() -> HitRecorder {
        Self{
            p: None,
            normal: None,
            material: None,
            t: 0.0,
            front_face: false
        }
    }
    pub(crate) fn set_face_normal(&mut self, ray:Ray, outward_normal:Vec3){
        let front_face = Vec3::dot(ray.direction(),outward_normal) < 0.0;
        if front_face {
            self.normal = Some(outward_normal);
            self.front_face = true;
        }else{
            self.normal = Some(- outward_normal);
            self.front_face = false;
        }
    }
}