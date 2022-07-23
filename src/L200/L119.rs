use crate::Solution;
impl Solution {
    fn fac(i: u128) -> u128 {
        match i {
            0 => 1,
            x => Self::fac(x - 1) * x,
        }
    }

    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut res = vec![];
        for i in 0..=row_index {
            res.push(
                (Self::fac(row_index as u128)
                    / (Self::fac(i as u128) * Self::fac((row_index - i) as u128)))
                    as i32,
            )
        }
        res
    }
}
