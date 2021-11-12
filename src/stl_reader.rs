use std::fs::File;
use std::io::Read;

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

    pub(crate)  fn read_angle_num(&mut self) -> Vec<u8>{
        self.index += 4;
        // let size =  self.buff[self.index -4..self.index];

        vec![]
    }
}
