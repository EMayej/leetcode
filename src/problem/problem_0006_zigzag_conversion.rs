/**
 *
 * https://leetcode.com/problems/zigzag-conversion
 *
 * 6. ZigZag Conversion
 *
 * The string `"PAYPALISHIRING"` is written in a zigzag pattern on a given number
 *of rows like this: (you may want to display this pattern in a fixed font for
 *better legibility)
 *
 *P   A   H   N
 *A P L S I I G
 *Y   I   R
 *
 *And then read line by line: `"PAHNAPLSIIGYIR"`
 *
 *Write the code that will take a string and make this conversion given a number
 *of rows:
 *
 *string convert(string s, int numRows);
 *
 *
 ***Example 1:**
 *
 ***Input:** s = "PAYPALISHIRING", numRows = 3
 ***Output:** "PAHNAPLSIIGYIR"
 *
 ***Example 2:**
 *
 ***Input:** s = "PAYPALISHIRING", numRows = 4
 ***Output:** "PINALSIGYAHRPI"
 ***Explanation:**
 *P     I    N
 *A   L S  I G
 *Y A   H R
 *P     I
 *
 ***Example 3:**
 *
 ***Input:** s = "A", numRows = 1
 ***Output:** "A"
 *
 *
 ***Constraints:**
 *
 ** `1 <= s.length <= 1000`
 ** `s` consists of English letters (lower-case and upper-case), `','` and `'.'`.
 ** `1 <= numRows <= 1000`
 *
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_6() {
        for case in vec![
            ("PAYPALISHIRING", 3, "PAHNAPLSIIGYIR"),
            ("PAYPALISHIRING", 4, "PINALSIGYAHRPI"),
            ("A", 1, "A"),
        ] {
            let (s, num_rows, wanted) = case;
            assert_eq!(Solution::convert(s.into(), num_rows), wanted);
        }
    }
}

struct Solution {}

// https://leetcode.com/problems/zigzag-conversion

enum Direction {
    Up,
    Down,
}
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows <= 1 {
            return s;
        }
        let mut rows = vec![vec![]; num_rows as usize];
        let mut i = 0;
        let mut direction = Direction::Down;
        for char in s.chars() {
            rows[i].push(char);

            match direction {
                Direction::Up => i -= 1,
                Direction::Down => i += 1,
            }

            if i == 0 {
                direction = Direction::Down;
            } else if i == rows.len() - 1 {
                direction = Direction::Up;
            }
        }
        rows.into_iter().flatten().collect()
    }
}
