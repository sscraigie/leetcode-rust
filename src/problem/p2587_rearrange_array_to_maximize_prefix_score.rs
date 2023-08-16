/**
 * [2587] Rearrange Array to Maximize Prefix Score
 *
 * You are given a 0-indexed integer array nums. You can rearrange the elements of nums to any order (including the given order).
 * Let prefix be the array containing the prefix sums of nums after rearranging it. In other words, prefix[i] is the sum of the elements from 0 to i in nums after rearranging it. The score of nums is the number of positive integers in the array prefix.
 * Return the maximum score you can achieve.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: nums = [2,-1,0,1,-3,3,-3]
 * Output: 6
 * Explanation: We can rearrange the array into nums = [2,3,1,-1,-3,0,-3].
 * prefix = [2,5,6,5,2,2,-1], so the score is 6.
 * It can be shown that 6 is the maximum score we can obtain.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: nums = [-2,-3,0]
 * Output: 0
 * Explanation: Any rearrangement of the array will result in a score of 0.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 10^5
 * 	-10^6 <= nums[i] <= 10^6
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/rearrange-array-to-maximize-prefix-score/
// discuss: https://leetcode.com/problems/rearrange-array-to-maximize-prefix-score/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_score(nums: Vec<i32>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2587() {
    }
}
