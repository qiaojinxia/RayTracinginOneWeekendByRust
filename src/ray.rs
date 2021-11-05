use crate::vec3::Vec3;

pub(crate) type Point3 = Vec3;

#[derive(Copy, Clone)]
pub(crate) struct Ray {
    origin: Point3,
    dir:Vec3,
}

impl Ray {
    pub(crate) fn form(origin: Point3, dir:Vec3) -> Self{
        Ray {
            origin,
            dir
        }
    }
    pub(crate) fn at(self,t:f64) -> Point3 {
        return self.origin + self.dir * t;
    }
    pub(crate) fn origin(self) -> Point3 {
        self.origin
    }
    pub(crate) fn direction(self) -> Vec3{
        self.dir
    }
}