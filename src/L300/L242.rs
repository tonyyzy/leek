use crate::Solution;
use std::collections::HashMap;

struct CharSet(HashMap<char, u64>);

impl CharSet {
    fn new(s: String) -> CharSet {
        let mut map = HashMap::new();
        for c in s.chars() {
            let k = map.entry(c).or_default();
            *k += 1
        }
        Self(map)
    }
}

impl PartialEq for CharSet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        CharSet::new(s) == CharSet::new(t)
    }
}
