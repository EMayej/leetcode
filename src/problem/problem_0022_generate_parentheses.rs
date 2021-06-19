/**
 *
 * https://leetcode.com/problems/generate-parentheses
 *
 * 22. Generate Parentheses
 *
 * Given `n` pairs of parentheses, write a function to *generate all combinations
 * of well-formed parentheses*.
 *
 *
 * **Example 1:**
 *
 * **Input:** n = 3
 * **Output:** ["((()))","(()())","(())()","()(())","()()()"]
 *
 * **Example 2:**
 *
 * **Input:** n = 1
 * **Output:** ["()"]
 *
 *
 * **Constraints:**
 *
 * * `1 <= n <= 8`
 *
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_22() {
        for case in vec![
            (3, vec!["((()))", "(()())", "(())()", "()(())", "()()()"]),
            (1, vec!["()"]),
        ] {
            let (n, wanted) = case;
            assert_eq!(Solution::generate_parenthesis(n), wanted);
        }
    }
}

struct Solution {}

// https://leetcode.com/problems/generate-parentheses

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut rv = Vec::new();
        Solution::dfs(n, n, &mut rv, &mut String::new());
        rv
    }

    fn dfs(left: i32, right: i32, rv: &mut Vec<String>, cur: &mut String) {
        if left == 0 && right == 0 {
            rv.push(cur.clone());
            return;
        }

        if left > 0 {
            cur.push('(');
            Solution::dfs(left - 1, right, rv, cur);
            cur.pop();
        }
        if left < right {
            cur.push(')');
            Solution::dfs(left, right - 1, rv, cur);
            cur.pop();
        }
    }
}
