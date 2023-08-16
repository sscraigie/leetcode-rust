/**
 * [1004] Max Consecutive Ones III
 *
 * Given a binary array nums and an integer k, return the maximum number of consecutive 1's in the array if you can flip at most k 0's.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: nums = [1,1,1,0,0,0,1,1,1,1,0], k = 2
 * Output: 6
 * Explanation: [1,1,1,0,0,<u>1,1,1,1,1,1</u>]
 * Bolded numbers were flipped from 0 to 1. The longest subarray is underlined.
 * <strong class="example">Example 2:
 * 
 * Input: nums = [0,0,1,1,0,0,1,1,1,0,1,1,0,0,0,1,1,1,1], k = 3
 * Output: 10
 * Explanation: [0,0,<u>1,1,1,1,1,1,1,1,1,1</u>,0,0,0,1,1,1,1]
 * Bolded numbers were flipped from 0 to 1. The longest subarray is underlined.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 10^5
 * 	nums[i] is either 0 or 1.
 * 	0 <= k <= nums.length
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/max-consecutive-ones-iii/
// discuss: https://leetcode.com/problems/max-consecutive-ones-iii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1004() {
    }
}
