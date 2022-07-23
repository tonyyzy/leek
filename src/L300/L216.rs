use crate::Solution;

impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let sel = vec![];
        let mut results = vec![];
        Self::find(k as usize, n, 1, sel, &mut results);
        results
    }

    fn find(k: usize, n: i32, num: i32, selection: Vec<i32>, results: &mut Vec<Vec<i32>>) {
        if selection.len() == k {
            if selection.iter().sum::<i32>() == n {
                results.push(selection);
            }
            return;
        }

        for i in num..=9 {
            let sum = selection.iter().sum::<i32>() + i;
            if sum > n {
                break;
            }

            let mut new_selection = selection.clone();
            new_selection.push(i);

            Self::find(k, n, i + 1, new_selection, results)
        }
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    #[test]
    fn too_small() {
        assert_eq!(Solution::combination_sum3(4, 1), vec![vec![]])
    }

    #[test]
    fn too_big() {
        assert_eq!(Solution::combination_sum3(1, 10), vec![vec![]])
    }

    #[test]
    fn t1() {
        assert_eq!(Solution::combination_sum3(3, 7), vec![vec![1, 2, 4]])
    }
}
