/**
 *
 * https://leetcode.com/problems/longest-palindromic-substring
 *
 * 5. Longest Palindromic Substring
 *
 * Given a string `s`, return *the longest palindromic substring* in `s`.
 *
 *
 ***Example 1:**
 *
 ***Input:** s = "babad"
 ***Output:** "bab"
 ***Note:** "aba" is also a valid answer.
 *
 ***Example 2:**
 *
 ***Input:** s = "cbbd"
 ***Output:** "bb"
 *
 ***Example 3:**
 *
 ***Input:** s = "a"
 ***Output:** "a"
 *
 ***Example 4:**
 *
 ***Input:** s = "ac"
 ***Output:** "a"
 *
 *
 ***Constraints:**
 *
 ** `1 <= s.length <= 1000`
 ** `s` consist of only digits and English letters (lower-case and/or upper-case),
 *
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_5() {
        for case in vec![("babad", "bab"), ("cbbd", "bb"), ("a", "a"), ("ac", "a")] {
            let (s, wanted) = case;
            assert_eq!(Solution::longest_palindrome(s.into()), wanted.to_string());
        }
    }
}

struct Solution {}

// https://leetcode.com/problems/longest-palindromic-substring

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.len() <= 1 {
            return s;
        }

        let mut max_left = 0;
        let mut max_right = 0;

        let data = s.as_bytes();
        let mut next_center = 0;
        while next_center < data.len() {
            let mut left = next_center;
            let mut right = next_center;
            while right < data.len() - 1 && data[right + 1] == data[next_center] {
                right += 1;
            }
            next_center = right + 1;

            while 0 < left && right < data.len() - 1 && data[left - 1] == data[right + 1] {
                left -= 1;
                right += 1;
            }

            if right - left > max_right - max_left {
                max_left = left;
                max_right = right;
            }
        }

        s[max_left..=max_right].into()
    }
}
