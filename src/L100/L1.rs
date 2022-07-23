use crate::Solution;

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for i in 0..nums.len() {
            let diff = target - nums[i];
            if let Some(v) = map.get(&diff) {
                return vec![*v, i as i32];
            } else {
                map.insert(nums[i], i as i32);
            }
        }
        vec![]
    }
}
