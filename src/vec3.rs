use std::ops::{Add, Sub, Mul, Neg, AddAssign, MulAssign, DivAssign, Div};
use crate::ray::Point3;

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

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Self::Output {
        Self {x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
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
    pub(crate) fn cross(u:&Vec3,v:&Vec3) -> Self{
        Vec3{
            x: u.x * v.z - u.z * v.y,
            y: u.z * v.x - u.x * v.z,
            z: u.x * v.y - u.y * v.x,
        }
    }
    pub(crate) fn unit_vector(self) -> Self{
        self / self.length()
    }
    pub(crate) fn dot(u: Point3, v: Point3) ->f64{
        u.x *v.x + u.y * v.y+ u.z * v.z
    }
}
