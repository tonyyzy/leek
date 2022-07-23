use crate::Solution;
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.is_empty() {
            return true;
        }
        let mut o = String::new();
        let mut s_iter = s.chars();
        let mut t_iter = t.chars();
        let mut sc = s_iter.next().unwrap();
        while let Some(c) = t_iter.next() {
            if c == sc {
                o.push(c);
                match s_iter.next() {
                    Some(x) => sc = x,
                    None => break,
                }
            }
        }

        o == s
    }
}
