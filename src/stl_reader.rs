use std::fs::File;
use std::io::Read;
use crate::common::{ parse_i32_little_endian, parse_f32_little_endian};
use crate::vec3::Vec3;
use std::sync::Arc;
use crate::shape::Triangle;
use crate::hit::Hittable;
use crate::material::Materials;
use crate::{point3};
use crate::Point3;
trait Reader{
    fn reader_next();
}

pub(crate) struct  StlReader{
    buff:Vec<u8>,
    index:usize,
}

impl StlReader{
    pub(crate) fn new_stl_reader(file_path:String) -> Self{
        let mut file = File::open(file_path).unwrap();
        let mut buffer:Vec<u8> = Vec::new();
        let _res = file.read_to_end(&mut buffer);
        Self{
            buff: buffer,
            index: 80
        }
    }

    pub(crate)  fn read_angle_num(&mut self) -> i32{
        self.index += 4;
        let size_content =  &self.buff[self.index -4..self.index];
        parse_i32_little_endian(size_content.to_owned())
    }

    pub(crate)  fn read_angle_point(&mut self) -> f64{
        self.index += 4;
        let size_content =  &self.buff[self.index -4..self.index];
        parse_f32_little_endian(size_content.to_owned()) as f64
    }

    pub(crate)  fn read_angle_info(&mut self){
        self.index += 2;
    }

    pub(crate) fn raed_all_shape_info(&mut self,objs:&mut Vec<Arc<dyn Hittable>>,material:Arc<dyn Materials>,angle:f64){
        let angle_num = self.read_angle_num();
        println!("三角形数量:{}",angle_num);
        for _i in 0..angle_num{
            let _n_x = self.read_angle_point();
            let _n_y = self.read_angle_point();
            let _n_z = self.read_angle_point();

            let t1_x = self.read_angle_point();
            let t1_y = self.read_angle_point();
            let t1_z = self.read_angle_point();
            let t2_x = self.read_angle_point();
            let t2_y = self.read_angle_point();
            let t2_z = self.read_angle_point();

            let t3_x = self.read_angle_point();
            let t3_y = self.read_angle_point();
            let t3_z = self.read_angle_point();
            let mut p1 = point3!(t1_x,t1_y,t1_z) ;
            let mut p2 = point3!(t2_x, t2_y, t2_z) ;
            let mut p3 = point3!(t3_x, t3_y, t3_z);
            let sin_theta = angle.sin();
            let cos_theta = angle.cos();
            p1 = Vec3::rotate_x(p1,angle.sin(),angle.cos());
            p2 = Vec3::rotate_x(p2,sin_theta,cos_theta);
            p3 = Vec3::rotate_x(p3,sin_theta,cos_theta);
            objs.push(Arc::new(Triangle::form(p1, p2,p3,material.clone())));
            self.read_angle_info();
        }
    }
}
