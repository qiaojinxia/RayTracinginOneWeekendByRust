extern crate rand;

use rand::Rng;
use std::sync::Arc;
use crate::hit::Hittable;

fn partition(items: &mut [Arc<dyn Hittable>], left: usize, right: usize,a:i32) -> usize {
    let mut random = rand::thread_rng();
    let mut i = left;
    let pivot_idx = random.gen_range(left, right + 1);
    let pivot = items[pivot_idx].get_center_point(a);
    items.swap(pivot_idx, right);
    for j in left..=right {
        if items[j].get_center_point(a) < pivot {
            items.swap(i, j);
            i += 1;
        }
    }
    items.swap(i, right);
    i
}

pub(crate) fn quick_select(items: &mut [Arc<dyn Hittable>], k: usize,a:i32) -> usize {
    let k = k - 1;  // k is 1-based index
    let mut left = 0;
    let mut right = items.len() - 1;
    while left <= right {
        let pivot = partition(items, left, right, a);
        if pivot == k {
            return pivot;
        } else if pivot > k {
            right = pivot - 1;
        } else {
            left = pivot + 1;
        }
    }
    panic!("error");
}
