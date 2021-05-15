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

        let mut max_first = 0;
        let mut max_last = 0;

        let data = s.as_bytes();
        let mut next_center = 0;
        while next_center < data.len() {
            let mut first = next_center;
            let mut last = next_center;
            while last < data.len() - 1 && data[last + 1] == data[next_center] {
                last += 1;
            }
            next_center = last + 1;

            while 0 < first && last < data.len() - 1 && data[first - 1] == data[last + 1] {
                first -= 1;
                last += 1;
            }

            if last - first > max_last - max_first {
                max_first = first;
                max_last = last;
            }
        }

        s[max_first..=max_last].into()
    }
}
