use crate::vec3::Vec3;

pub(crate) type Point3 = Vec3;

impl Vec3{
    pub(crate) fn new() -> Self{
        Vec3{
            x:0.0,
            y:0.0,
            z:0.0
        }
    }
    pub(crate) fn form(x:f64,y:f64,z:f64) -> Self{
        Vec3{
            x,
            y,
            z
        }
    }
}
#[derive(Copy, Clone)]
pub(crate) struct Ray {
    origin: Point3,
    dir:Vec3,
}

impl Ray {
    pub(crate) fn new() -> Self{
        Ray {
            origin:Point3::new(),
            dir:Point3::new(),
        }
    }
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