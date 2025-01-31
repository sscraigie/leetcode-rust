/**
 * [523] Continuous Subarray Sum
 *
 * Given an integer array nums and an integer k, return true if nums has a good subarray or false otherwise.
 * A good subarray is a subarray where:
 * 
 * 	its length is at least two, and
 * 	the sum of the elements of the subarray is a multiple of k.
 * 
 * Note that:
 * 
 * 	A subarray is a contiguous part of the array.
 * 	An integer x is a multiple of k if there exists an integer n such that x = n * k. 0 is always a multiple of k.
 * 
 *  
 * <strong class="example">Example 1:
 * 
 * Input: nums = [23,<u>2,4</u>,6,7], k = 6
 * Output: true
 * Explanation: [2, 4] is a continuous subarray of size 2 whose elements sum up to 6.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: nums = [<u>23,2,6,4,7</u>], k = 6
 * Output: true
 * Explanation: [23, 2, 6, 4, 7] is an continuous subarray of size 5 whose elements sum up to 42.
 * 42 is a multiple of 6 because 42 = 7 * 6 and 7 is an integer.
 * 
 * <strong class="example">Example 3:
 * 
 * Input: nums = [23,2,6,4,7], k = 13
 * Output: false
 * 
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 10^5
 * 	0 <= nums[i] <= 10^9
 * 	0 <= sum(nums[i]) <= 2^31 - 1
 * 	1 <= k <= 2^31 - 1
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/continuous-subarray-sum/
// discuss: https://leetcode.com/problems/continuous-subarray-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_523() {
    }
}
