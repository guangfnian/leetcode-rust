use std::cmp::max;
use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut mp = HashMap::new();
        let mut ans = 0;
        let mut l = 0;
        let mut r = 0;
        let b = s.as_bytes();
        while r < b.len() {
            match mp.get(&b[r]) {
                Some(_) => {
                    ans = max(ans, r - l);
                    mp.remove(&b[l]);
                    l += 1;
                }
                None => {
                    mp.insert(b[r], true);
                    r += 1;
                }
            };
        }
        max(ans, r - l) as i32
    }
}

#[cfg(test)]
mod test {
    use crate::solution::longest_substring_without_repeating_characters::Solution;

    #[test]
    fn test() {
        let tests = [("abcabcbb", 3), ("bbbbb", 1), ("pwwkew", 3), ("bcba", 3)];
        for (index, value) in tests.iter().enumerate() {
            assert_eq!(
                value.1,
                Solution::length_of_longest_substring(String::from(value.0))
            );
        }
    }
}
