/**
 *
 * https://leetcode.com/problems/longest-substring-without-repeating-characters
 *
 * 3. Longest Substring Without Repeating Characters
 *
 * Given a string `s`, find the length of the longest substring without repeating
 *characters.
 *
 *
 ***Example 1:**
 *
 ***Input:** s = "abcabcbb"
 ***Output:** 3
 ***Explanation:** The answer is "abc", with the length of 3.
 *
 ***Example 2:**
 *
 ***Input:** s = "bbbbb"
 ***Output:** 1
 ***Explanation:** The answer is "b", with the length of 1.
 *
 ***Example 3:**
 *
 ***Input:** s = "pwwkew"
 ***Output:** 3
 ***Explanation:** The answer is "wke", with the length of 3.
 *Notice that the answer must be a substring, "pwke" is a subsequence and not a su
 *bstring.
 *
 ***Example 4:**
 *
 ***Input:** s = ""
 ***Output:** 0
 *
 *
 ***Constraints:**
 *
 ** `0 <= s.length <= 5 * 104`
 ** `s` consists of English letters, digits, symbols and spaces.
 *
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3() {
        for case in vec![("abcabcbb", 3), ("bbbbb", 1), ("pwwkew", 3), ("", 0)] {
            let (s, wanted) = case;
            assert_eq!(Solution::length_of_longest_substring(s.to_owned()), wanted);
        }
    }
}

struct Solution {}

// https://leetcode.com/problems/longest-substring-without-repeating-characters

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut last_indexes = [-1; 1 << 8];
        let mut first = 0;
        let mut longest = 0;

        for (i, c) in s.as_bytes().iter().enumerate() {
            let last = last_indexes[*c as usize];
            last_indexes[*c as usize] = i as i32;

            if last >= first {
                first = last + 1
            }

            longest = longest.max(i as i32 - first + 1);
        }

        longest
    }
}
