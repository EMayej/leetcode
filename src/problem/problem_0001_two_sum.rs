/**
 *
 * https://leetcode.com/problems/two-sum
 *
 * 1. Two Sum
 *
 * Given an array of integers `nums` and an integer `target`, return *indices of
 *the two numbers such that they add up to `target`*.
 *
 *You may assume that each input would have ***exactly* one solution**, and you
 *may not use the *same* element twice.
 *
 *You can return the answer in any order.
 *
 *
 ***Example 1:**
 *
 ***Input:** nums = [2,7,11,15], target = 9
 ***Output:** [0,1]
 ***Output:** Because nums[0] + nums[1] == 9, we return [0, 1].
 *
 ***Example 2:**
 *
 ***Input:** nums = [3,2,4], target = 6
 ***Output:** [1,2]
 *
 ***Example 3:**
 *
 ***Input:** nums = [3,3], target = 6
 ***Output:** [0,1]
 *
 *
 ***Constraints:**
 *
 ** `2 <= nums.length <= 103`
 ** `-109 <= nums[i] <= 109`
 ** `-109 <= target <= 109`
 ** **Only one valid answer exists.**
 *
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        for case in vec![
            (vec![2, 7, 11, 15], 9, vec![0, 1]),
            (vec![3, 2, 4], 6, vec![1, 2]),
            (vec![3, 3], 6, vec![0, 1]),
        ] {
            let (nums, target, wanted) = case;
            assert_eq!(Solution::two_sum(nums, target), wanted);
        }
    }
}

struct Solution {}

/// START SOLUTION
use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut visited = HashMap::with_capacity(nums.len());
        for (index, num) in nums.iter().enumerate() {
            match visited.get(&(target - num)) {
                None => {
                    visited.insert(num, index);
                }
                Some(visited_index) => return vec![*visited_index as i32, index as i32],
            }
        }
        vec![]
    }
}
