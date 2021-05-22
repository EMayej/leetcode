/**
 *
 * https://leetcode.com/problems/reverse-integer
 *
 * 7. Reverse Integer
 *
 * Given a signed 32-bit integer `x`, return `x`* with its digits reversed*. If
 *reversing `x` causes the value to go outside the signed 32-bit integer range
 *`[-2^31, 2^31 - 1]`, then return `0`.
 *
 ***Assume the environment does not allow you to store 64-bit integers (signed or
 *unsigned).**
 *
 *
 ***Example 1:**
 *
 ***Input:** x = 123
 ***Output:** 321
 *
 ***Example 2:**
 *
 ***Input:** x = -123
 ***Output:** -321
 *
 ***Example 3:**
 *
 ***Input:** x = 120
 ***Output:** 21
 *
 ***Example 4:**
 *
 ***Input:** x = 0
 ***Output:** 0
 *
 *
 ***Constraints:**
 *
 ** `-2^31 <= x <= 2^31 - 1`
 *
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_7() {
        for case in vec![(123, 321), (-123, -321), (120, 21), (0, 0)] {
            let (x, wanted) = case;
            assert_eq!(Solution::reverse(x), wanted);
        }
    }
}

struct Solution {}

// https://leetcode.com/problems/reverse-integer

impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        let mut rv = 0;

        let max_without_last_digit = i32::MAX / 10;
        let last_digit_of_max = i32::MAX % 10;
        let min_without_last_digit = i32::MIN / 10;
        let last_digit_of_min = i32::MIN % 10;
        while x != 0 {
            let digit = x % 10;
            x /= 10;

            if rv > max_without_last_digit
                || (rv == max_without_last_digit && digit > last_digit_of_max)
            {
                return 0;
            }
            if rv < min_without_last_digit
                || (rv == min_without_last_digit && digit < last_digit_of_min)
            {
                return 0;
            }

            rv = rv * 10 + digit;
        }
        rv
    }
}
