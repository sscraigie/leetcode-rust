/**
 * [2334] Subarray With Elements Greater Than Varying Threshold
 *
 * You are given an integer array nums and an integer threshold.
 * Find any subarray of nums of length k such that every element in the subarray is greater than threshold / k.
 * Return the size of any such subarray. If there is no such subarray, return -1.
 * A subarray is a contiguous non-empty sequence of elements within an array.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: nums = [1,3,4,3,1], threshold = 6
 * Output: 3
 * Explanation: The subarray [3,4,3] has a size of 3, and every element is greater than 6 / 3 = 2.
 * Note that this is the only valid subarray.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: nums = [6,5,6,5,8], threshold = 7
 * Output: 1
 * Explanation: The subarray [8] has a size of 1, and 8 > 7 / 1 = 7. So 1 is returned.
 * Note that the subarray [6,5] has a size of 2, and every element is greater than 7 / 2 = 3.5. 
 * Similarly, the subarrays [6,5,6], [6,5,6,5], [6,5,6,5,8] also satisfy the given conditions.
 * Therefore, 2, 3, 4, or 5 may also be returned.
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 10^5
 * 	1 <= nums[i], threshold <= 10^9
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/subarray-with-elements-greater-than-varying-threshold/
// discuss: https://leetcode.com/problems/subarray-with-elements-greater-than-varying-threshold/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn valid_subarray_size(nums: Vec<i32>, threshold: i32) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2334() {
    }
}
