
#[macro_export]
macro_rules! point3 {
    ( $a:expr,$b:expr,$c:expr ) => {
        Point3::form($a as f64,$b as f64 ,$c as f64)
    };
}

#[macro_export]
macro_rules! color3 {
    ( $a:expr,$b:expr,$c:expr ) => {
        Point3::form($a as f64,$b as f64 ,$c as f64)
    };
}