/**
 *
 * https://leetcode.com/problems/3sum-closest
 *
 * 16. 3Sum Closest
 *
 * Given an array `nums` of *n* integers and an integer `target`, find three
 * integers in `nums` such that the sum is closest to `target`. Return the sum of
 * the three integers. You may assume that each input would have exactly one
 * solution.
 *
 *
 * **Example 1:**
 *
 * **Input:** nums = [-1,2,1,-4], target = 1
 * **Output:** 2
 * **Explanation:** The sum that is closest to the target is 2. (-1 + 2 + 1 = 2).
 *
 *
 * **Constraints:**
 *
 * * `3 <= nums.length <= 10^3`
 * * `-10^3 <= nums[i] <= 10^3`
 * * `-10^4 <= target <= 10^4`
 *
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_16() {
        for case in vec![
            (vec![-1, 2, 1, -4], 1, 2),
            (vec![1, 2, 3], 1, 6),
            (vec![1, 2, 4, 8, 16, 32, 64, 128], 82, 82),
        ] {
            let (nums, target, wanted) = case;
            assert_eq!(wanted, Solution::three_sum_closest(nums, target));
        }
    }
}

struct Solution {}

// https://leetcode.com/problems/3sum-closest

impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort_unstable();

        let mut closest_delta = i32::MAX;
        let mut closest_sum = 0;
        for i in 0..nums.len() - 2 {
            let mut first = i + 1;
            let mut last = nums.len() - 1;

            while first < last {
                let sum = nums[i] + nums[first] + nums[last];
                let delta = sum - target;
                if delta < 0 {
                    first += 1;
                } else if delta > 0 {
                    last -= 1;
                } else {
                    return target;
                }

                let delta = delta.abs();
                if delta < closest_delta {
                    closest_delta = delta;
                    closest_sum = sum;
                }
            }
        }
        closest_sum
    }
}
