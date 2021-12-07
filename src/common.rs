use rand::{thread_rng, Rng};
use std::convert::TryFrom;
use std::cmp::Ordering;
use std::cmp::Ordering::{Less, Equal, Greater};
use crate::shape::AABB;
use crate::vec3::Vec3;
use std::f64::consts::PI;
use crate::{point3};
use crate::ray::Point3;


pub enum Tuple{
    UV(f64,f64)
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
        }
    }

}
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


pub(crate) fn rand_i3_range(min:i32,max:i32) -> i32{
    let mut rng = thread_rng();
    let i = rng.gen_range(min,max);
    i
}

pub(crate) fn surrounding_box(a:AABB,b:AABB) -> Option<AABB>{
    let min_x = f64::min(a.minimum.x,b.minimum.x);
    let min_y = f64::min(a.minimum.y,b.minimum.y);
    let min_z = f64::min(a.minimum.z,b.minimum.z);
    let max_x = f64::max(a.maximum.x,b.maximum.x);
    let max_y = f64::max(a.maximum.y,b.maximum.y);
    let max_z = f64::max(a.maximum.z,b.maximum.z);
    Some(AABB::form(point3!(min_x,min_y,min_z),point3!(max_x,max_y,max_z)))
}



pub(crate) struct Perlin{
    ranvec:Vec<Vec3>,
    perm_x:Vec<i32>,
    perm_y:Vec<i32>,
    perm_z:Vec<i32>,
}

impl Perlin{
    pub(crate) fn new() -> Self{
        let mut ranvec = vec![];
        for _i in 0.. 256{
            ranvec.push(Vec3::random_range(-1.0,1.0).unit_vector());
        }
         Perlin{
            ranvec: ranvec,
            perm_x: Self::perlin_generate_perm(),
            perm_y: Self::perlin_generate_perm(),
            perm_z: Self::perlin_generate_perm()
        }
    }
    pub(crate) fn noise(&self,p:&Point3) -> f64 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();
        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;
        let mut c =  vec![];
        for di in 0 .. 2 {
            let mut v_di = vec![];
            for dj in 0 .. 2 {
                let mut v_dj = vec![];
                for dk in 0 .. 2 {
                    let index = self.perm_x[((i+di)  & 255) as usize] ^ self.perm_y[((j+dj) & 255) as usize] ^ self.perm_z[((k+dk) & 255) as usize];
                    let val = self.ranvec[index as usize];
                    v_dj.push(val);
                }
                v_di.push(v_dj);
            }
            c.push(v_di);
        }
        return Self::trilinear_interp(c, u, v, w);

    }
    fn trilinear_interp(c:Vec<Vec<Vec<Vec3>>>,u:f64,v:f64,w:f64) -> f64{
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;
        for i in 0..2{
            for j in 0..2{
                for k in 0..2{
                    let weight_v = point3!(u-i  as f64,v-j as f64,w-k as f64);
                    accum += (i as f64 *uu + (1-i) as f64 *(1.0-uu))*
                        (j as f64 * vv + (1-j) as f64 *(1.0-vv))*
                        (k as f64 * ww + (1-k) as f64 *(1.0 -ww)) * Vec3::dot(c[i][j][k],weight_v);
                }
            }
        }
        accum
    }
    fn  perlin_generate_perm()  -> Vec<i32>{
        let mut p = vec![];
        for i in  0..256{
            p.push(i);
        }
        Self::permute(&mut p, 256);
        p
    }

    pub(crate) fn turb(&self, mut p:Point3, depth:i32) -> f64{
        let mut accum = 0.0;
        let mut weight = 1.0;
        for _i in 0 ..depth {
            accum += weight * self.noise(&p);
            weight *= 0.5;
            p *= 2.0;
        }
        return accum.abs();
    }
    fn permute(p:&mut Vec<i32>,n:i32) {
        for i in  0 .. n - 1 {
            let re_i = n -1 -i;
            let target = rand_i3_range(0, re_i);
            let tmp = p[re_i as usize];
            p[re_i as usize] = p[target as usize];
            p[target as usize] = tmp;
        }
    }
}



pub(crate) fn except(n:i32, p:f64) -> f64{
    n as f64 * (1.0 - p) * p.powi(n-1)
}