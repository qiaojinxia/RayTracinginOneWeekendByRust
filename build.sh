#!/bin/sh
file_name="image.ppm"
rm -rf $file_name
echo "P3
       800 800
      255" >> $file_name
search(){
int=0
for file in file*
    do
        echo "合并线程$int 渲染图像 $file"
        cat $file >> $file_name
        let "int++"
        rm -rf $file
    done
}
search .
