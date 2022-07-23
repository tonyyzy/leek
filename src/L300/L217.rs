use crate::Solution;
use std::collections::HashSet;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut s = HashSet::new();
        for num in nums {
            if !s.insert(num) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn t1() {
        use super::Solution;

        let _ = Solution{};
        let nums = [1, 1, 1, 3, 3, 4, 3, 2, 4, 2].to_vec();
        assert!(Solution::contains_duplicate(nums))
    }
}
