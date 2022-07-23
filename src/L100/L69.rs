use crate::Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut n_prev = 2.0;
        let mut n = Self::equa(n_prev, x as f32);
        while (n - n_prev).abs() > 0.1 {
            n_prev = n;
            n = Self::equa(n_prev, x as f32);
            println!("{} {}", n_prev, n)
        }

        n.trunc() as i32
    }

    fn equa(x: f32, a: f32) -> f32 {
        0.5 * x + 0.5 * a / x
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        println!("{}", Solution::my_sqrt(124354241));
    }
}