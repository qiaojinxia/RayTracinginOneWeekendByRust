use rand::{thread_rng, Rng};

pub(crate) fn clamp(x:f64, min:f64, max:f64) ->f64{
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

