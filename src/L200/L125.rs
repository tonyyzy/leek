use crate::Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let sani = s
            .chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect::<Vec<char>>();
        sani.iter().zip(sani.iter().rev()).all(|(a, b)| a == b)
    }
}
