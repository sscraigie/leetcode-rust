/**
 * [1330] Reverse Subarray To Maximize Array Value
 *
 * You are given an integer array nums. The value of this array is defined as the sum of |nums[i] - nums[i + 1]| for all 0 <= i < nums.length - 1.
 * You are allowed to select any subarray of the given array and reverse it. You can perform this operation only once.
 * Find maximum possible value of the final array.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: nums = [2,3,1,5,4]
 * Output: 10
 * Explanation: By reversing the subarray [3,1,5] the array becomes [2,5,1,3,4] whose value is 10.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: nums = [2,4,9,24,2,1,10]
 * Output: 68
 * 
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 3 * 10^4
 * 	-10^5 <= nums[i] <= 10^5
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/reverse-subarray-to-maximize-array-value/
// discuss: https://leetcode.com/problems/reverse-subarray-to-maximize-array-value/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_value_after_reverse(nums: Vec<i32>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1330() {
    }
}
