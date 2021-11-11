use crate::ray::Ray;
use crate::hit::HitRecorder;
use crate::Color;
use crate::vec3::Vec3;
use crate::common::rand_f64;


pub(crate) trait Materials:Send + Sync{
    fn scatter(&self,ray_in:&Ray,rec:HitRecorder) -> Option<Ray>;
    fn get_color(&self) -> Color;
}

pub(crate) struct Lambertian{
    pub(crate) albedo:Color,
}

impl Lambertian{
    pub(crate) fn form(r:f64,g:f64,b:f64) -> Lambertian{
        Self{
            albedo: Vec3::form(r,g,b),
        }
    }
}

impl Materials for Lambertian{
    fn scatter(&self, _ray_in: &Ray, rec: HitRecorder) -> Option<Ray> {
        let mut scatter_direction = rec.normal.unwrap() + Vec3::random_unit_vector();
        if scatter_direction.near_zero(){
            scatter_direction = rec.normal.unwrap();
        }
        return Some(Ray::form(rec.p.unwrap(), scatter_direction))
    }

    fn get_color(&self) -> Color {
        self.albedo
    }
}

pub(crate) struct Metal{
    albedo:Color,
    fuzz:f64,
}

impl Metal{
    pub(crate) fn form(r:f64, g:f64, b:f64, mut f:f64) -> Metal{
        if f > 1.0{
            f = 1.0
        }
        Self{
            fuzz:f,
            albedo: Color::form(r,g,b),
        }
    }
}
impl Materials for Metal{
    fn scatter(&self, ray_in: &Ray, rec: HitRecorder) -> Option<Ray> {
        let reflected = Vec3::reflect(ray_in.direction().unit_vector(),rec.normal.unwrap());
        let scattered = Ray::form(rec.p.unwrap(), reflected + Vec3::random_in_unit_sphere() * self.fuzz  );
        let x = Vec3::dot(scattered.direction(),rec.normal.unwrap());
        if  x > 0.0{
           return Some(scattered);
        }
        None
    }

    fn get_color(&self) -> Color {
        self.albedo
    }
}

pub(crate) struct Dielectric{
    ir:f64
}

impl Dielectric{
    pub(crate) fn form(ir:f64) -> Self{
        Self{
            ir
        }
    }
}

impl Materials for Dielectric{
    fn scatter(&self, ray_in: &Ray, rec: HitRecorder) -> Option<Ray> {
       let mut refraction_ratio = self.ir;
       if rec.front_face {
           refraction_ratio = 1.0 / self.ir
       }
        let unit_direction = ray_in.direction().unit_vector();
        let cos_theta = f64::min(Vec3::dot(-unit_direction,rec.normal.unwrap()),1.0);
        //根据折射率的公式:𝜂/𝜂' * sin𝜃 = sin'𝜃 从折射率搞得地方 折射到折射率低的地方 1.5 / 1.0 * sin𝜃 => 1.5 * sin𝜃 = sin'𝜃 等式两边的值域 不相同 等式不成立
        //所以 不能用折射公式 这个时候我们要使用 反射公式
        let sin_theta = 1.0 - (cos_theta * cos_theta);
            let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction;
        if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > rand_f64() {
            direction = Vec3::reflect(unit_direction, rec.normal.unwrap());

        }else{
             direction = Vec3::refract(unit_direction,
                                         rec.normal.unwrap(),refraction_ratio);
        }
        Some(Ray::form(rec.p.unwrap(),direction))
    }

    fn get_color(&self) -> Color {
        Color::form(1.0,1.0,1.0)
    }
}

impl Dielectric{
    pub(crate) fn reflectance(cosine:f64,ref_idx:f64) -> f64{
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
    }
}