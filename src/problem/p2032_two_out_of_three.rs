/**
 * [2032] Two Out of Three
 *
 * Given three integer arrays nums1, nums2, and nums3, return a distinct array containing all the values that are present in at least two out of the three arrays. You may return the values in any order.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: nums1 = [1,1,3,2], nums2 = [2,3], nums3 = [3]
 * Output: [3,2]
 * Explanation: The values that are present in at least two arrays are:
 * - 3, in all three arrays.
 * - 2, in nums1 and nums2.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: nums1 = [3,1], nums2 = [2,3], nums3 = [1,2]
 * Output: [2,3,1]
 * Explanation: The values that are present in at least two arrays are:
 * - 2, in nums2 and nums3.
 * - 3, in nums1 and nums2.
 * - 1, in nums1 and nums3.
 * 
 * <strong class="example">Example 3:
 * 
 * Input: nums1 = [1,2,2], nums2 = [4,3,3], nums3 = [5]
 * Output: []
 * Explanation: No value is present in at least two arrays.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= nums1.length, nums2.length, nums3.length <= 100
 * 	1 <= nums1[i], nums2[j], nums3[k] <= 100
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/two-out-of-three/
// discuss: https://leetcode.com/problems/two-out-of-three/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn two_out_of_three(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>) -> Vec<i32> {
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2032() {
    }
}
