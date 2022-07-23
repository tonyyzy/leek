impl crate::Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        if digits.len() == 0 {
            return digits;
        }
        let mut res = digits.clone();
        let mut pos = res.len().checked_sub(1);
        while let Some(p) = pos {
            res[p] += 1;
            if res[p] == 10 {
                res[p] = 0;
                pos = p.checked_sub(1)
            } else {
                return res;
            }
        }
        let mut one = vec![1];
        one.append(&mut res);
        one
    }
}
