use crate::Solution;

use std::collections::HashSet;
use std::iter::FromIterator;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut max = 0;
        let set: HashSet<i32> = HashSet::from_iter(nums);
        for n in set.iter() {
            // test for start of block
            if !set.contains(&(n - 1)) {
                let mut length = 1;
                let mut cur = n + 1;
                while set.contains(&cur) {
                    length += 1;
                    cur += 1
                }
                max = max.max(length)
            }
        }
        max
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn t1() {
        assert_eq!(Solution::longest_consecutive(vec![1, 2, 3]), 3);
    }

    #[test]
    fn t2() {
        assert_eq!(Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
    }

    #[test]
    fn t3() {
        assert_eq!(
            Solution::longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]),
            9
        );
    }
}
