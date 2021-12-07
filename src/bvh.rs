use crate::hit::{Hittable, HitRecorder};
use crate::ray::{Point3, Ray};
use crate::shape::AABB;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use crate::common::{surrounding_box, Axis};
use std::borrow::{BorrowMut, Borrow};
use crate::sort::quick_select;
use crate::vec3::Vec3;

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

    fn bounding_box(&self) -> Option<AABB> {
        self.bbox
    }

    fn get_center_point(&self, a: &Axis) -> f64 {
        let center_point = (self.bbox.unwrap().maximum +  self.bbox.unwrap().minimum) / 2.0;
        a.call(center_point)
    }

    fn pdf_value(&self, rec: &mut HitRecorder, p: Point3, dir: Vec3) -> f64 {
        todo!()
    }


    fn random_sample(&self) -> Vec3 {
        todo!()
    }
}
impl BvhNode{
    pub(crate) fn form(rc_objects: &mut [Arc<dyn Hittable>],t0:f64,t1:f64) -> Option<BvhNode>{
        if rc_objects.len() <= 5{
            let mut vec = vec![];
            let mut total_box = None;
            let mut first = true;
            let mut objs_num = 0;
            for obj in rc_objects{
                let tmp_box = obj.clone().bounding_box();
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
        let mut max_box = None;
        let mut first = true;
        //计算包围盒 优先划分 长度比较长的包围盒
        for v in rc_objects.iter(){
            if first {
                max_box = v.bounding_box();
                first = false;
                continue;
            }
            max_box = surrounding_box(max_box.unwrap(),v.bounding_box().unwrap())
        }
        let x_gap = (max_box.unwrap().maximum.x - max_box.unwrap().minimum.x).abs();
        let y_gap = (max_box.unwrap().maximum.y - max_box.unwrap().minimum.y).abs();
        let z_gap = (max_box.unwrap().maximum.z - max_box.unwrap().minimum.z).abs();
        let axis;
        if x_gap >= y_gap && x_gap>= z_gap{
            axis = Axis::X;
        }else if y_gap >= z_gap && y_gap >= x_gap{
            axis = Axis::Y;
        }else {
            axis = Axis::Z;
        }
        let mut mid = (rc_objects.len() / 2 + 1) as usize;
        mid = quick_select(rc_objects, mid,axis.borrow());
        let left = Self::form(rc_objects[..mid].borrow_mut(), t0,t1);
        let right = Self::form(rc_objects[mid..].borrow_mut(),  t0,t1);
        let xbox = surrounding_box(left.as_ref().unwrap().bounding_box().unwrap(),
                                   right.as_ref().unwrap().bounding_box().unwrap());
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