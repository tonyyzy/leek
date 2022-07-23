use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut cache = HashMap::new();
        let mut candidates = candidates;
        candidates.sort();
        let mut res = Self::sums(&candidates, target, &mut cache);
        res.sort();
        res.dedup();
        res
    }

    fn sums(
        candidates: &[i32],
        target: i32,
        cache: &mut HashMap<i32, Vec<Vec<i32>>>,
    ) -> Vec<Vec<i32>> {
        if let Some(x) = cache.get(&target) {
            return x.clone();
        }
        let mut res = vec![];
        if target < candidates[0] {
            return vec![];
        }
        if candidates.contains(&target) {
            res.push(vec![target])
        }

        for x in 0..candidates.len() {
            for mut v in Self::sums(candidates, target - candidates[x], cache) {
                if !v.is_empty() {
                    v.push(candidates[x]);
                    v.sort();
                    res.push(v)
                }
            }
        }
        cache.insert(target, res.clone());
        res
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn t1() {
        println!("{:?}", Solution::combination_sum(vec![2], 1));
    }
}
