/**
 * [2815] Max Pair Sum in an Array
 *
 * You are given a 0-indexed integer array nums. You have to find the maximum sum of a pair of numbers from nums such that the maximum digit in both numbers are equal.
 * Return the maximum sum or -1 if no such pair exists.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: nums = [51,71,17,24,42]
 * Output: 88
 * Explanation: 
 * For i = 1 and j = 2, nums[i] and nums[j] have equal maximum digits with a pair sum of 71 + 17 = 88. 
 * For i = 3 and j = 4, nums[i] and nums[j] have equal maximum digits with a pair sum of 24 + 42 = 66.
 * It can be shown that there are no other pairs with equal maximum digits, so the answer is 88.
 * <strong class="example">Example 2:
 * 
 * Input: nums = [1,2,3,4]
 * Output: -1
 * Explanation: No pair exists in nums with equal maximum digits.
 * 
 *  
 * Constraints:
 * 
 * 	2 <= nums.length <= 100
 * 	1 <= nums[i] <= 10^4
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/max-pair-sum-in-an-array/
// discuss: https://leetcode.com/problems/max-pair-sum-in-an-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_sum(nums: Vec<i32>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2815() {
    }
}
