//! Solution for https://leetcode.com/problems/repeated-substring-pattern
//! 459. Repeated Substring Pattern
//!

impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let str_bytes = s.into_bytes();
        let str_bytes_len = str_bytes.len();

        for len in 1..=str_bytes_len / 2 {
            if str_bytes[..len].repeat(str_bytes_len / len) == str_bytes {
                return true;
            }
        }

        false
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

// Example 1:

// Assumptions
// aa aa
// aco aco
// ba ab
// a a (?)
//
// Input: s = "abab"
// Output: true
// Explanation: It is the substring "ab" twice.
// Example 2:
//
// Input: s = "aba"
// "aaa" = true
// "abbba"
// Output: false
// Example 3:
//
// Input: s = "abcabcabcabc"
// Output: true
// Explanation: It is the substring "abc" four times or the substring "abcabc" twice.

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("abab", true)]
    #[case("aba", false)]
    #[case("abcabcabcabc", true)]
    fn case(#[case] s: String, #[case] expected: bool) {
        let actual = Solution::repeated_substring_pattern(s);
        assert_eq!(actual, expected);
    }
}
