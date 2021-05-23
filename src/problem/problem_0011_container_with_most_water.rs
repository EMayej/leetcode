/**
 *
 * https://leetcode.com/problems/container-with-most-water
 *
 * 11. Container With Most Water
 *
 * Given `n` non-negative integers `a1, a2, ..., an` , where each represents a
 *point at coordinate `(i, ai)`. `n` vertical lines are drawn such that the two
 *endpoints of the line `i` is at `(i, ai)` and `(i, 0)`. Find two lines, which,
 *together with the x-axis forms a container, such that the container contains the
 *most water.
 *
 ***Notice** that you may not slant the container.
 *
 *
 ***Example 1:**
 *
 *[]
 ***Input:** height = [1,8,6,2,5,4,8,3,7]
 ***Output:** 49
 ***Explanation:** The above vertical lines are represented by array [1,8,6,2,5,4,
 *8,3,7]. In this case, the max area of water (blue section) the container can con
 *tain is 49.
 *
 ***Example 2:**
 *
 ***Input:** height = [1,1]
 ***Output:** 1
 *
 ***Example 3:**
 *
 ***Input:** height = [4,3,2,1,4]
 ***Output:** 16
 *
 ***Example 4:**
 *
 ***Input:** height = [1,2,1]
 ***Output:** 2
 *
 *
 ***Constraints:**
 *
 ** `n == height.length`
 ** `2 <= n <= 10^5`
 ** `0 <= height[i] <= 10^4`
 *
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_11() {
        for case in vec![
            (vec![1, 8, 6, 2, 5, 4, 8, 3, 7], 49),
            (vec![1, 1], 1),
            (vec![4, 3, 2, 1, 4], 16),
            (vec![1, 2, 1], 2),
        ] {
            let (height, wanted) = case;
            assert_eq!(Solution::max_area(height), wanted);
        }
    }
}

struct Solution {}

// https://leetcode.com/problems/container-with-most-water

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        if height.len() <= 1 {
            return 0;
        }

        let mut volume = 0;
        let mut left = 0;
        let mut right = height.len() - 1;
        while left < right {
            let width = (right - left) as i32;
            if height[left] < height[right] {
                volume = volume.max(width * height[left]);
                left += 1;
            } else {
                volume = volume.max(width * height[right]);
                right -= 1;
            }
        }
        volume
    }
}
