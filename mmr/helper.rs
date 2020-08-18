//! Helper fns

use std::cmp::Ordering;

fn log2_floor(mut num: i64) -> i64 {
    let mut res = -1;
    while num > 0 {
        res += 1;
        num >>= 1;
    }
    res
}

/// MMR size to last leaf `O(log2(log2(n)))`
pub fn mmr_size_to_last_leaf(mmr_size: i64) -> i64 {
    if mmr_size == 0 {
        return 0;
    }

    let mut m = log2_floor(mmr_size);
    loop {
        match (2 * m - m.count_ones() as i64).cmp(&mmr_size) {
            Ordering::Equal => return m - 1,
            Ordering::Greater => m -= 1,
            Ordering::Less => m += 1,
        }
    }
}
