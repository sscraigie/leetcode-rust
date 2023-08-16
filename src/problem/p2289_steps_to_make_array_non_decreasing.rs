/**
 * [2289] Steps to Make Array Non-decreasing
 *
 * You are given a 0-indexed integer array nums. In one step, remove all elements nums[i] where nums[i - 1] > nums[i] for all 0 < i < nums.length.
 * Return the number of steps performed until nums becomes a non-decreasing array.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: nums = [5,3,4,4,7,3,6,11,8,5,11]
 * Output: 3
 * Explanation: The following are the steps performed:
 * - Step 1: [5,<u>3</u>,4,4,7,<u>3</u>,6,11,<u>8</u>,<u>5</u>,11] becomes [5,4,4,7,6,11,11]
 * - Step 2: [5,<u>4</u>,4,7,<u>6</u>,11,11] becomes [5,4,7,11,11]
 * - Step 3: [5,<u>4</u>,7,11,11] becomes [5,7,11,11]
 * [5,7,11,11] is a non-decreasing array. Therefore, we return 3.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: nums = [4,5,7,7,13]
 * Output: 0
 * Explanation: nums is already a non-decreasing array. Therefore, we return 0.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 10^5
 * 	1 <= nums[i] <= 10^9
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/steps-to-make-array-non-decreasing/
// discuss: https://leetcode.com/problems/steps-to-make-array-non-decreasing/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn total_steps(nums: Vec<i32>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2289() {
    }
}