use rand::{thread_rng, Rng};

const PI: f64 = 3.1415926535897932385;
pub(crate) const MIN: f64 = 1e-8;

pub(crate) fn degrees_to_radians(degrees:f64) -> f64{
    degrees * PI  / 180.0
}

pub(crate) fn clamp(x:f64, min:f64, max:f64) -> f64{
    if x < min {return min};
    if x > max {return max};
    return x;
}

pub(crate) fn rand_f64() -> f64{
    let mut rng = thread_rng();
    let i = rng.gen();
    i
}

pub(crate) fn rand_range_f64(min:f64,max:f64) -> f64{
    let mut rng = thread_rng();
    let i = rng.gen_range(min,max);
    i
}

pub(crate) fn f64_near_zero(n:f64) -> bool{
    let s = 1e-8;
    if n <= s{
        return true
    }
    return false;
}