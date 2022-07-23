use crate::Solution;

use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        let mut s: Vec<Option<Vec<i32>>> = vec![None; nums.len() + 1];
        for n in nums {
            *map.entry(n).or_insert(0) += 1;
        }

        for (k, v) in map.into_iter() {
            s[v].get_or_insert(vec![]).push(k)
        }

        s.into_iter()
            .rev()
            .filter(|a| a.is_some())
            .flat_map(|a| a.unwrap().into_iter())
            .take(k as usize)
            .collect()
    }
}
