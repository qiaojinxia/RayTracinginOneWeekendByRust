use std::sync::Arc;
use crate::hit::{Hittable, HitRecorder};
use crate::ray::{Point3, Ray};
use crate::shape::AABB;
use crate::common::{surrounding_box, Axis};
use crate::vec3::Vec3;

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
    pub(crate) fn add_objs(&mut self, objs:Vec<Arc<dyn Hittable>>){
        for obj in objs.iter(){
            self.objects.push(obj.clone());
        }
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
    fn bounding_box(&self) -> Option<AABB> {
        let mut first_box = true;
        let mut tmp_box = None;
        for obj in self.objects.iter(){
            let out_box= obj.clone().bounding_box();
            if first_box {
                tmp_box = out_box;
                first_box = false;
                continue;
            }
            tmp_box = surrounding_box(tmp_box.unwrap(),out_box.unwrap())
        }
        tmp_box
    }

    fn get_center_point(&self, _a: &Axis) -> f64 {
        todo!()
    }

    fn pdf_value(&self, rec: &mut HitRecorder, p: Point3, dir: Vec3) -> f64 {
        todo!()
    }


    fn random_sample(&self) -> Vec3 {
        todo!()
    }
}
