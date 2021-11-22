use crate::ray::Point3;
use crate::Color;
use std::sync::Arc;
use crate::common::Perlin;

pub(crate) trait Texture:Send + Sync{
    fn value(&self,u:f64,v:f64,p:&Point3) -> Color;
}

pub(crate) struct SolidColor{
    color_value:Color
}

impl SolidColor{
    pub(crate) fn form(r:f64,g:f64,b:f64) -> Self{
        SolidColor{
            color_value: Color::form(r,g,b)
        }
    }

    pub(crate) fn form_color(color:Color) -> Self{
        SolidColor{
            color_value: color
        }
    }
}
impl Texture for SolidColor{
    fn value(&self,_u: f64, _v: f64, _p: &Point3) -> Color {
        self.color_value
    }
}


pub(crate) struct CheckerTexture{
    odd:Option<Arc<dyn Texture>>,
    even:Option<Arc<dyn Texture>>,
}

impl CheckerTexture{
    pub(crate) fn form(even:Arc<dyn Texture>,odd:Arc<dyn Texture>) -> Self{
        Self{
            odd: Some(odd),
            even: Some(even)
        }
    }

    pub(crate) fn form_color(even:Color,odd:Color) -> Self{
        Self{
            odd: Some(Arc::new(SolidColor::form_color(odd))),
            even: Some(Arc::new(SolidColor::form_color(even)))
        }
    }
}

impl Texture for CheckerTexture{
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let sines = (p.x * 10.0).sin() * (p.y * 10.0).sin() * (p.z * 10.0).sin();
        if sines < 0.0 {
            return self.odd.clone().unwrap().value(u,v,p);
        }
        self.even.clone().unwrap().value(u,v,p)

    }
}


pub(crate) struct NoiseTexture{
    noise:Perlin,
    scale:f64,
}

impl NoiseTexture{
    pub(crate) fn new() -> Self{
        Self{
            noise: Perlin::new(),
            scale:1.0,
        }
    }

    pub(crate) fn form(s:f64) -> Self{
        Self{
            noise: Perlin::new(),
            scale:s,
        }
    }

}

impl Texture for NoiseTexture{
    fn value(&self, _u: f64, _v: f64, p: &Point3) -> Color {
        let p1 = *p * self.scale;
        Color::form(1.0,1.0,1.0) * 0.5 * (1.0 + (self.scale * p.z + 10.0 * self.noise.turb(p1,7)).sin())
    }
}