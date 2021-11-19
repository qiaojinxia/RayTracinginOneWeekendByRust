use rand::{thread_rng, Rng};
use std::convert::TryFrom;
use std::cmp::Ordering;
use std::cmp::Ordering::{Less, Equal, Greater};
use crate::shape::AABB;
use crate::ray::Point3;
use crate::vec3::Vec3;

pub enum Common{
    UV(f64,f64),
    None
}

pub enum Axis{
    X,
    Y,
    Z,
}

impl Axis{
    pub(crate) fn call(&self,vec:Vec3) -> f64{
        match self {
            Axis::X => {vec.x}
            Axis::Y => {vec.y}
            Axis::Z => {vec.z}
            _ => {panic!("错误的轴!")}
        }
    }

}

const PI: f64 = 3.1415926535897932385;
pub(crate) const MIN: f64 = 1e-10;

pub(crate) fn degrees_to_radians(degrees:f64) -> f64{
    degrees * PI  / 180.0
}

pub(crate) fn clamp(x:f64, min:f64, max:f64) -> f64{
    if x < min {return min};
    if x > max {return max};
    return x;
}

pub(crate) fn rand_f64() -> f64{
    let mut rng = thread_rng();
    let i = rng.gen();
    i
}

pub(crate) fn rand_range_f64(min:f64,max:f64) -> f64{
    let mut rng = thread_rng();
    let i = rng.gen_range(min,max);
    i
}

pub(crate) fn parse_i32_little_endian(bytes:Vec<u8>) -> i32{
         ((bytes[3] as i32) << 24) +
         ((bytes[2] as i32) << 16) +
         ((bytes[1] as i32) << 8)  +
         (bytes[0] as i32)

}


pub(crate) fn parse_f32_little_endian(bytes:Vec<u8>) -> f32{
    f32::from_le_bytes(<[u8; 4]>::try_from(bytes).unwrap())
}

pub(crate) fn f64_near_zero(n:f64) -> bool{
    let s = 1e-8;
    if n <= s && n >= -s {
        return true
    }
    return false;
}


pub(crate) fn cmp_f64(f1:f64,f2:f64) -> Ordering{
    if f1 < f2 {
        return Less
    }else if (f1 - f2).abs() <= MIN{
        return Equal
    }
    return Greater
}

pub(crate) fn rand_i32() -> i32{
    let mut rng = thread_rng();
    let i = rng.gen_range(0,2);
    i
}

pub(crate) fn surrounding_box(a:AABB,b:AABB) -> Option<AABB>{
    let min_x = f64::min(a.minimum.x,b.minimum.x);
    let min_y = f64::min(a.minimum.y,b.minimum.y);
    let min_z = f64::min(a.minimum.z,b.minimum.z);
    let max_x = f64::max(a.maximum.x,b.maximum.x);
    let max_y = f64::max(a.maximum.y,b.maximum.y);
    let max_z = f64::max(a.maximum.z,b.maximum.z);
    Some(AABB::form(Point3::form(min_x,min_y,min_z),Point3::form(max_x,max_y,max_z)))
}