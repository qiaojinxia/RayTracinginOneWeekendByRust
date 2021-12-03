use std::f64::consts::PI;
use std::ops::{Add, Sub, Mul, Neg, AddAssign, MulAssign, DivAssign, Div};
use crate::ray::Point3;
use crate::common::{rand_range_f64, rand_f64};


#[derive(Debug,Copy, Clone)]
pub(crate) struct Vec3{
    pub(crate) x:f64,
    pub(crate) y:f64,
    pub(crate) z:f64,
}


impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Self::Output {
        Self { x: self.x + other.x, y: self.y + other.y , z: self.z + other.z }
    }

}

impl Add<f64> for Vec3 {
    type Output = Vec3;
    fn add(self, shr: f64) -> Self::Output {
        Self { x: self.x + shr, y: self.y + shr , z: self.z + shr }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Self::Output {
        Self {x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
    }
}

impl Sub<f64> for Vec3 {
    type Output = Vec3;
    fn sub(self, shr: f64) -> Self::Output {
        Self { x: self.x - shr, y: self.y - shr , z: self.z - shr }
    }
}


impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {x: self.x * rhs.x, y: self.y  * rhs.y, z: self.z * rhs.z}
    }

}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Self {x: self.x * rhs, y: self.y  * rhs, z: self.z * rhs}
    }

}
impl Mul<i32> for Vec3 {
    type Output = Vec3;
    fn mul(self, i: i32) -> Self::Output {
        let m = i as f64;
        Self {x: self.x * m, y: self.y  * m , z: self.z * m}
    }
}

impl Div<f64> for Vec3{
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self{
        Self{ x:self.x / rhs, y:self.y / rhs, z:self.z / rhs,}

    }
}

impl Neg for Vec3{
    type Output = Vec3;
    fn neg(self) -> Self {
        Self {x: -self.x , y: -self.y , z: -self.z}
    }
}

impl AddAssign for Vec3{
    fn add_assign(&mut self, rhs: Self) {
       self.x =  self.x + rhs.x;
       self.y =  self.y + rhs.y;
       self.z =  self.z + rhs.z;
    }
}

impl MulAssign for Vec3{
    fn mul_assign(&mut self, rhs: Self) {
        self.x =  self.x * rhs.x;
        self.y =  self.y * rhs.y;
        self.z =  self.z * rhs.z;
    }
}

impl MulAssign<f64> for Vec3{
    fn mul_assign(&mut self, rhs: f64) {
        self.x =  self.x * rhs;
        self.y =  self.y * rhs;
        self.z =  self.z * rhs;
    }
}
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1 as f64 /rhs
    }
}

impl DivAssign for Vec3{
    fn div_assign(&mut self, rhs: Self) {
        self.x =  self.x / rhs.x;
        self.y =  self.y / rhs.y;
        self.z =  self.z / rhs.z;
    }
}

impl Vec3{
    pub(crate) fn length(self) -> f64{
        self.length_squared().sqrt()
    }
    pub(crate) fn length_squared(self) -> f64{
        return self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub(crate) fn cross(u:Vec3,v:Vec3) -> Self{
        Vec3{
            x: u.y * v.z - u.z * v.y,
            y: u.z * v.x - u.x * v.z,
            z: u.x * v.y - u.y * v.x,
        }
    }
    pub(crate) fn unit_vector(self) -> Self{
        self / self.length()
    }

    pub(crate) fn get_field(self, index:i32) -> f64{
       if index == 0 {
           return self.x;
       }else if index == 1{
           return self.y;
       }else if index == 2{
           return self.z;
       }
        panic!("错误索引!")
    }

    pub(crate) fn set_i_field(&mut self,index:i32,val:f64){
        if index == 0 {
            self.x = val ;
        }else if index == 1{
            self.y = val ;
        }else if index == 2{
            self.z = val ;
        }
    }

    pub(crate) fn dot(u: Point3, v: Point3) ->f64{
        u.x *v.x + u.y * v.y+ u.z * v.z
    }

    pub(crate) fn random() -> Self{
        return Vec3::form(rand_f64(),rand_f64(),rand_f64())
    }

    pub(crate) fn random_range(min:f64,max:f64) -> Self{
        return Vec3::form(rand_range_f64(min,max),rand_range_f64(min,max),rand_range_f64(min,max))
    }

    pub(crate) fn random_in_unit_sphere() -> Vec3{
        loop{
            let p = Vec3::random_range(-1.0,1.0);
            if p.length_squared() >= 1.0{
                continue
            }
            return p;
        }
    }

    pub(crate) fn random_unit_vector() -> Vec3{
        Self::random_in_unit_sphere().unit_vector()
    }
    pub(crate) fn random_in_hemisphere(normal:Vec3) -> Vec3{
        let in_unit_sphere = Vec3::random_in_unit_sphere();
        return if Vec3::dot(in_unit_sphere, normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub(crate) fn near_zero(self) -> bool{
        let s = 1e-8;
        return (self.x).abs() < s && (self.y).abs() < s && (self.z).abs() < s
    }

    pub(crate) fn reflect(v:Vec3,n:Vec3) -> Vec3{
        v - n * (Vec3::dot(v,n) * 2.0)
    }

    pub(crate) fn refract(uv:Vec3, n:Vec3, eta:f64) -> Vec3{
        let cos_theta = Vec3::dot(-uv,n);
        let cos_theta2 = 1.0 - eta * eta * (1.0 - cos_theta * cos_theta);
        let t = uv * eta + n *(eta * cos_theta - cos_theta2.abs().sqrt());
        return t ;
    }

    pub(crate) fn random_in_unit_disk() -> Vec3{
        loop {
            let p = Vec3::form(rand_range_f64(-1.0,1.0),rand_range_f64(-1.0,1.0),0.0);
            if p.length_squared() >= 1.0{continue}
            return p;
        }
    }

    pub(crate) fn min(v1:Vec3,v2:Vec3) -> Vec3{
        let mut tmp = Vec3::new();
        tmp.x = f64::min(v1.x,v2.x);
        tmp.y = f64::min(v1.y,v2.y);
        tmp.z = f64::min(v1.z,v2.z);
        tmp
    }


    pub(crate) fn max(v1:Vec3,v2:Vec3) -> Vec3{
        let mut tmp = Vec3::new();
        tmp.x = f64::max(v1.x,v2.x);
        tmp.y = f64::max(v1.y,v2.y);
        tmp.z = f64::max(v1.z,v2.z);
        tmp

    }

    pub(crate) fn rotate_x(v1:Vec3,sin_theta:f64,cos_theta:f64) -> Self{
        let r1 = Vec3::form(1.0,0.0,0.0);
        let r2 = Vec3::form(0.0,cos_theta,-sin_theta);
        let r3 = Vec3::form(0.0,sin_theta,cos_theta);
        let x = Vec3::dot(r1 ,v1);
        let y = Vec3::dot(r2 ,v1);
        let z = Vec3::dot(r3 ,v1);
        Self{
            x,
            y,
            z
        }
    }

    pub(crate) fn rotate_y(v1:Vec3,sin_theta:f64,cos_theta:f64) -> Self{
        let r1 = Vec3::form(cos_theta,0.0,sin_theta);
        let r2 = Vec3::form(0.0,1.0,0.0);
        let r3 = Vec3::form(-sin_theta,0.0,cos_theta);
        let x = Vec3::dot(r1 ,v1);
        let y = Vec3::dot(r2 ,v1);
        let z = Vec3::dot(r3 ,v1);
        Self{
            x,
            y,
            z
        }
    }

    pub(crate) fn rotate_z(v1:Vec3,radians:f64) -> Self{
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let r1 = Vec3::form(cos_theta,-sin_theta,0.0);
        let r2 = Vec3::form(sin_theta,cos_theta,0.0);
        let r3 = Vec3::form(0.0,0.0,1.0);
        let x = Vec3::dot(r1 ,v1);
        let y = Vec3::dot(r2 ,v1);
        let z = Vec3::dot(r3 ,v1);
        Self{
            x,
            y,
            z
        }
    }
    pub(crate) fn random_uniform() -> Self{
        let z = rand_f64();
        let s = rand_f64();
        let r = (1.0 - z * z).sqrt();
        let phi = 2.0 * PI * s;
        let x = phi.cos() * r;
        let y = phi.sin() * r;
        Self::form(x, z, y)
    }

}
