use crate::Solution;

impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut candidates = candidates;
        candidates.sort();
        let mut res = Self::sumz(candidates, 0, target);
        res.sort();
        res.dedup();
        res
    }

    fn sumz(candidates: Vec<i32>, last: i32, target: i32) -> Vec<Vec<i32>> {
        let sum = candidates.iter().sum::<i32>();
        if sum < target {
            return vec![];
        } else if sum == target {
            return vec![candidates]
        }
        // println!("start {:?} {}", candidates, target);
        if candidates.is_empty() || candidates[0] > target {
            return vec![];
        } else if candidates[0] == target {
            return vec![vec![candidates[0]]]
        }
        let mut res = vec![];
        for (i, candidate) in candidates.iter().enumerate() {
            if *candidate < last {
                continue;
            }
            if *candidate == target {
                res.push(vec![target]);
                continue;
            }
            let mut inter = candidates.clone();
            inter.remove(i);
            if target - candidate > 0 {
                for mut each in Self::sumz(inter, *candidate, target - candidate) {
                    let mut r = vec![*candidate];
                    r.append(&mut each);
                    res.push(r)
                }
            }
        }
        // println!("return {:?}", res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn t1() {
        println!(
            "{:?}",
            Solution::combination_sum2(vec![1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1], 30)
        );
    }
}
