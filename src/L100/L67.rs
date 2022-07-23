use crate::Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let res = isize::from_str_radix(&a, 2).unwrap() + isize::from_str_radix(&b, 2).unwrap();
        format!("{:b}", res)
    }
}
