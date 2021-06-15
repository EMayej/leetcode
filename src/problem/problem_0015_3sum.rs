/**
 *
 * https://leetcode.com/problems/3sum
 *
 * 15. 3Sum
 *
 * Given an integer array nums, return all the triplets `[nums[i], nums[j],
 * nums[k]]` such that `i != j`, `i != k`, and `j != k`, and `nums[i] + nums[j] +
 * nums[k] == 0`.
 *
 * Notice that the solution set must not contain duplicate triplets.
 *
 *
 * **Example 1:**
 *
 * **Input:** nums = [-1,0,1,2,-1,-4]
 * **Output:** [[-1,-1,2],[-1,0,1]]
 *
 * **Example 2:**
 *
 * **Input:** nums = []
 * **Output:** []
 *
 * **Example 3:**
 *
 * **Input:** nums = [0]
 * **Output:** []
 *
 *
 * **Constraints:**
 *
 * * `0 <= nums.length <= 3000`
 * * `-105 <= nums[i] <= 105`
 *
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_15() {
        for case in vec![(
            vec![-1, 0, 1, 2, -1, -4],
            vec![vec![-1, -1, 2], vec![-1, 0, 1]],
        )] {
            let (nums, wanted) = case;
            assert_eq!(Solution::three_sum(nums), wanted);
        }
    }
}

struct Solution {}

// https://leetcode.com/problems/3sum

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 {
            return vec![];
        }

        let mut rv = Vec::new();

        nums.sort_unstable();
        for i in 0..nums.len() - 2 {
            // deduplicate
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let mut first = i + 1;
            let mut last = nums.len() - 1;

            while first < last {
                let sum = nums[i] + nums[first] + nums[last];
                if sum < 0 {
                    first += 1
                } else if sum > 0 {
                    last -= 1
                } else {
                    rv.push(vec![nums[i], nums[first], nums[last]]);

                    // deduplicate
                    while first < last && nums[first] == nums[first + 1] {
                        first += 1;
                    }
                    while first < last && nums[last] == nums[last - 1] {
                        last -= 1;
                    }

                    first += 1;
                    last -= 1;
                }
            }
        }

        rv
    }
}
