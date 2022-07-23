use crate::Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![1; nums.len()];
        let mut reg = nums[0];
        for (ind, val) in nums[..(nums.len() - 1)].iter().enumerate() {
            res[ind] = reg;
            reg *= val;
        }
        let mut reg = nums[nums.len() - 1];
        for (ind, val) in nums[1..].iter().enumerate().rev() {
            res[ind] = res[ind] * reg;
            reg *= val;
        }
        res
    }
}
