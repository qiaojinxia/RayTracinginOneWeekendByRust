use crate::ray::{Point3, Ray};
use crate::vec3::Vec3;

pub(crate) struct hit_recorder{
    pub(crate) p:Point3,
    pub(crate) normal:Vec3,
    pub(crate) t:f64,
}

pub(crate) trait Hit{
    fn hit(self,ray:Ray,t_min:f64,t_max:f64,rec:hit_recorder) -> bool;
}
