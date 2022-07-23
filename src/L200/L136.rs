use crate::Solution;

use std::collections::HashSet;
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut hs = HashSet::new();
        for n in nums {
            if !hs.remove(&n) {
                hs.insert(n);
            }
        }
        *hs.iter().next().unwrap()
    }
}
