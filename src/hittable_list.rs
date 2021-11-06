use std::sync::Arc;
use crate::hit::{Hittable, HitRecorder};
use crate::ray::Ray;
pub(crate) struct HittableList{
    pub(crate) objects:Vec<Arc<dyn Hittable>>,
}

impl HittableList{
    pub(crate) fn new() -> Self{
        Self{ objects: vec![] }
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
}
