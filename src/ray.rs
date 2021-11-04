use crate::vec3::Vec3;

pub(crate) type point3  = Vec3;

pub(crate) struct Ray {
    origin:point3,
    dir:Vec3,
}

impl Ray {
    pub(crate) fn form(origin:point3,dir:Vec3) -> Self{
        Ray {
            origin,
            dir
        }
    }
    pub(crate) fn at(self,t:f64) -> point3{
        return self.origin + self.dir * t;
    }
    pub(crate) fn origin(self) -> point3 {
        self.origin
    }
    pub(crate) fn direction(self) -> Vec3{
        self.dir
    }
}