/**
 *
 * https://leetcode.com/problems/palindrome-number
 *
 * 9. Palindrome Number
 *
 * Given an integer `x`, return `true` if `x` is palindrome integer.
 *
 *An integer is a **palindrome** when it reads the same backward as forward. For
 *example, `121` is palindrome while `123` is not.
 *
 *
 ***Example 1:**
 *
 ***Input:** x = 121
 ***Output:** true
 *
 ***Example 2:**
 *
 ***Input:** x = -121
 ***Output:** false
 ***Explanation:** From left to right, it reads -121. From right to left, it becom
 *es 121-. Therefore it is not a palindrome.
 *
 ***Example 3:**
 *
 ***Input:** x = 10
 ***Output:** false
 ***Explanation:** Reads 01 from right to left. Therefore it is not a palindrome.
 *
 ***Example 4:**
 *
 ***Input:** x = -101
 ***Output:** false
 *
 *
 ***Constraints:**
 *
 ** `-2^31 <= x <= 2^31 - 1`
 *
 *
 ***Follow up:** Could you solve it without converting the integer to a string?
 *
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_9() {
        for case in vec![(121, true), (-121, false), (10, false), (-101, false)] {
            let (x, wanted) = case;
            assert_eq!(Solution::is_palindrome(x), wanted);
        }
    }
}

struct Solution {}

// https://leetcode.com/problems/palindrome-number

impl Solution {
    pub fn is_palindrome(mut x: i32) -> bool {
        if x < 0 || (x % 10 == 0 && x != 0) {
            return false;
        }

        let mut reversed = 0;
        while x > reversed {
            reversed = reversed * 10 + x % 10;
            x /= 10;
        }

        x == reversed || x == reversed / 10
    }
}
