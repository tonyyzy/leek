use crate::Solution;
impl Solution {
    pub fn group_anagrams(mut strs: Vec<String>) -> Vec<Vec<String>> {
        use std::collections::HashMap;

        strs.drain(..)
            .fold(HashMap::new(), |mut acc, s| {
                let mut k = s.chars().collect::<Vec<char>>();
                k.sort();
                acc.entry(k).or_insert(Vec::new()).push(s);
                acc
            })
            .drain()
            .map(|(_, v)| v)
            .collect()
    }
}
