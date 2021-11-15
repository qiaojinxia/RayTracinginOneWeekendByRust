use crate::hit::{Hittable, HitRecorder};
use crate::ray::Ray;
use crate::shape::AABB;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use crate::common::{ rand_i32, surrounding_box};
use std::borrow::{BorrowMut};
use crate::sort::quick_select;

pub(crate) struct BvhNode{
    src_objects:Option<Vec<Arc<dyn Hittable>>>,
    contains_objs:i32,
    bbox:Option<AABB>,
    pub(crate) left:Option<Arc<BvhNode>>,
    pub(crate) right:Option<Arc<BvhNode>>,
}

impl Debug for BvhNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{:?}",self)
    }
}

impl Hittable for BvhNode {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, rec: &mut HitRecorder) -> bool {
        if !self.bbox.as_ref().unwrap().hit(ray, t_min, t_max, rec) {
            return false;
        }
        let mut is_hit = false;
        let mut tt_max = t_max;
        match self.src_objects {
            None => {}
            Some(ref shapes) => {
                for obj in shapes.iter() {
                    if obj.hit(ray, t_min, tt_max, rec) {
                        tt_max = rec.t;
                        is_hit = true
                    }
                }
                return is_hit;
            }
        }
        let left_hit = self.left.as_ref().unwrap().clone().hit(ray, t_min, t_max, rec);
        let mut max_t = t_max;
        //如果有一边命中了 另一边 如果要命中 这个t 要比左边小 所以传入 左边计算的T方便右边去比较
        if left_hit {
            max_t = f64::min(max_t,rec.t);
        }
        let right_hit = self.right.as_ref().unwrap().clone().hit(ray, t_min, max_t, rec);
        return left_hit || right_hit;
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        self.bbox
    }

    fn get_center_point(&self, _dir: i32) -> f64 {
        todo!()
    }
}
impl BvhNode{
    pub(crate) fn form(rc_objects: &mut [Arc<dyn Hittable>],t0:f64,t1:f64, axis:i32) -> Option<BvhNode>{
        if rc_objects.len() <= 10{
            let mut vec = vec![];
            let mut total_box = None;
            let mut first = true;
            let mut objs_num = 0;
            for obj in rc_objects{
                let tmp_box = obj.clone().bounding_box(t0,t1);
                objs_num += 1;
                vec.push(obj.clone());
                if first {
                    total_box = tmp_box;
                    first = false;
                    continue;
                }
                total_box = surrounding_box(total_box.unwrap(),tmp_box.unwrap());
            }
            return Some(Self{
                src_objects: Some(vec),
                contains_objs: objs_num,
                bbox: total_box,
                left: None,
                right: None,
            })
        }
        let mut mid = (rc_objects.len() / 2 + 1) as usize;
        mid = quick_select(rc_objects, mid,axis);
        let left = Self::form(rc_objects[..mid].borrow_mut(), t0,t1,rand_i32());
        let right = Self::form(rc_objects[mid..].borrow_mut(),  t0,t1,rand_i32());
        let xbox = surrounding_box(left.as_ref().unwrap().bounding_box(t0, t1).unwrap(),
                                   right.as_ref().unwrap().bounding_box(t0, t1).unwrap());
        let total_num = left.as_ref().unwrap().contains_objs + right.as_ref().unwrap().contains_objs;
        Some(Self{
            src_objects: None,
            contains_objs: total_num,
            bbox: xbox,
            left: Some(Arc::new(left.unwrap())),
            right: Some(Arc::new(right.unwrap()))
        })
    }
}