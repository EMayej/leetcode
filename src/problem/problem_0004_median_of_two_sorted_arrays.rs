/**
 *
 * https://leetcode.com/problems/median-of-two-sorted-arrays
 *
 * 4. Median of Two Sorted Arrays
 *
 * Given two sorted arrays `nums1` and `nums2` of size `m` and `n` respectively,
 *return **the median** of the two sorted arrays.
 *
 *The overall run time complexity should be `O(log (m+n))`.
 *
 *
 ***Example 1:**
 *
 ***Input:** nums1 = [1,3], nums2 = [2]
 ***Output:** 2.00000
 ***Explanation:** merged array = [1,2,3] and median is 2.
 *
 ***Example 2:**
 *
 ***Input:** nums1 = [1,2], nums2 = [3,4]
 ***Output:** 2.50000
 ***Explanation:** merged array = [1,2,3,4] and median is (2 + 3) / 2 = 2.5.
 *
 ***Example 3:**
 *
 ***Input:** nums1 = [0,0], nums2 = [0,0]
 ***Output:** 0.00000
 *
 ***Example 4:**
 *
 ***Input:** nums1 = [], nums2 = [1]
 ***Output:** 1.00000
 *
 ***Example 5:**
 *
 ***Input:** nums1 = [2], nums2 = []
 ***Output:** 2.00000
 *
 *
 ***Constraints:**
 *
 ** `nums1.length == m`
 ** `nums2.length == n`
 ** `0 <= m <= 1000`
 ** `0 <= n <= 1000`
 ** `1 <= m + n <= 2000`
 ** `-10^6 <= nums1[i], nums2[i] <= 10^6`
 *
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_4() {
        for case in vec![
            (vec![1, 3], vec![2, 7], 2.5),
            (vec![1, 3], vec![2], 2.0),
            (vec![1, 2], vec![3, 4], 2.5),
            (vec![0, 0], vec![0, 0], 0.0),
            (vec![], vec![1], 1.0),
            (vec![2], vec![], 2.0),
        ] {
            let (nums1, nums2, wanted) = case;
            assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), wanted);
        }
    }
}

struct Solution {}

// https://leetcode.com/problems/median-of-two-sorted-arrays

///
/// nums1: ... nums1[i - 1] , nums1[i] ... nums1[n - 1]
/// nums2: ... nums2[j - 1], nums2[j] ... nums2[m - 1]
///
/// The following conditions should always be true:
/// 1. i + j == n - i + m - j
/// 2. nums1[i - 1] <= nums2[j] && nums2[j - 1] <= nums1[i]
///
/// We use binary search to find i in nums1.
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        // Ensure nums1 is shorter so `j` doesn't point out of nums2.
        if nums1.len() > nums2.len() {
            return Self::find_median_sorted_arrays(nums2, nums1);
        }

        let n = nums1.len();
        let m = nums2.len();

        // add 1 to n + m to ensure the median is the max_left if (n + m) is odd
        let half = (n + m + 1) / 2;

        let mut begin = 0;
        let mut end = n;
        while begin < end {
            let i = begin + (end - begin) / 2;
            let j = half - i;
            if i > 0 && nums1[i - 1] > nums2[j] {
                end = i - 1;
            } else if i < n && nums2[j - 1] > nums1[i] {
                begin = i + 1;
            } else {
                begin = i;
                end = i;
            }
        }

        let i = begin;
        let j = half - i;
        let max_left = if i == 0 {
            nums2[j - 1]
        } else if j == 0 {
            nums1[i - 1]
        } else {
            nums1[i - 1].max(nums2[j - 1])
        };
        if (n + m) % 2 == 1 {
            return max_left as f64;
        }

        let min_right = if i == n {
            nums2[j]
        } else if j == m {
            nums1[i]
        } else {
            nums1[i].min(nums2[j])
        };
        (max_left + min_right) as f64 / 2.0
    }
}
