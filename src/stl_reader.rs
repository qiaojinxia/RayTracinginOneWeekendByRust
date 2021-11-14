use std::fs::File;
use std::io::Read;
use crate::common::{ parse_i32_little_endian, parse_f32_little_endian};

trait Reader{
    fn ReaderNext();
}

pub(crate) struct  StlReader{
    buff:Vec<u8>,
    index:usize,
}

impl StlReader{
    pub(crate) fn newStlReader(filePath:String) -> Self{
        let mut file = File::open(filePath).unwrap();
        let mut buffer:Vec<u8> = Vec::new();
        file.read_to_end(&mut buffer);
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
}
