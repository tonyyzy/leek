use crate::Solution;
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        if num_rows >= 1 {
            res.push(vec![1])
        }
        if num_rows >= 2 {
            res.push(vec![1, 1])
        }
        if num_rows > 2 {
            for r in 2..num_rows as usize {
                let mut temp = vec![1];
                for i in 0..(res[r - 1].len() - 1) {
                    temp.push(res[r - 1][i] + res[r - 1][i + 1])
                }
                temp.push(1);
                res.push(temp)
            }
        }
        res
    }
}
