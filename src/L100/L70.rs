use std::{collections::HashMap};

use crate::Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut cache = HashMap::new();
        cache.insert(1, 1);
        cache.insert(0, 1);
        Self::ways(n, &mut cache)
    }

    fn ways(n: i32, cache: &mut HashMap<i32, i32>) -> i32 {
        if let Some(x) = cache.get(&n) {
            *x
        } else {
            let res = Self::ways(n - 1, cache) + Self::ways(n - 2, cache);
            cache.insert(n, res);
            res
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        println!("{}", Solution::climb_stairs(44))
    }
}
