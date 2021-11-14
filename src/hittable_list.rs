use std::sync::Arc;
use crate::hit::{Hittable, HitRecorder};
use crate::ray::Ray;
use crate::shape::AABB;
use crate::common::surrounding_box;

#[derive(Debug, Clone)]
pub(crate) struct HittableList{
    pub(crate) objects:Vec<Arc<dyn Hittable>>,
}

impl HittableList{
    pub(crate) fn new() -> Self{
        Self{ objects: vec![]}
    }
    pub(crate) fn add(&mut self, obj:Arc<dyn Hittable>){
        self.objects.push(obj);
    }
}


impl Hittable for HittableList{
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, rec: &mut HitRecorder) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for obj in self.objects.iter(){
            if obj.hit(ray,t_min,closest_so_far,rec){
                hit_anything = true;
                closest_so_far = rec.t;
            }
        }
        return hit_anything
    }
    //计算 整个场景的最大包围盒
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        let mut first_box = true;
        let mut tmp_box = None;
        for obj in self.objects.iter(){
            let mut out_box= obj.clone().bounding_box(t0,t1);
            if first_box {
                tmp_box = out_box;
                first_box = false;
                continue;
            }
            tmp_box = surrounding_box(tmp_box.unwrap(),out_box.unwrap())
        }
        tmp_box
    }

    fn get_axis(&self, s: i32) -> f64 {
        todo!()
    }
}
