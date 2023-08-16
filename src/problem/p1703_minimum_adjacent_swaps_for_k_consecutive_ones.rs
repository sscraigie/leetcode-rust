/**
 * [1703] Minimum Adjacent Swaps for K Consecutive Ones
 *
 * You are given an integer array, nums, and an integer k. nums comprises of only 0's and 1's. In one move, you can choose two adjacent indices and swap their values.
 * Return the minimum number of moves required so that nums has k consecutive 1's.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: nums = [1,0,0,1,0,1], k = 2
 * Output: 1
 * Explanation: In 1 move, nums could be [1,0,0,0,<u>1</u>,<u>1</u>] and have 2 consecutive 1's.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: nums = [1,0,0,0,0,0,1,1], k = 3
 * Output: 5
 * Explanation: In 5 moves, the leftmost 1 can be shifted right until nums = [0,0,0,0,0,<u>1</u>,<u>1</u>,<u>1</u>].
 * 
 * <strong class="example">Example 3:
 * 
 * Input: nums = [1,1,0,1], k = 2
 * Output: 0
 * Explanation: nums already has 2 consecutive 1's.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 10^5
 * 	nums[i] is 0 or 1.
 * 	1 <= k <= sum(nums)
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-adjacent-swaps-for-k-consecutive-ones/
// discuss: https://leetcode.com/problems/minimum-adjacent-swaps-for-k-consecutive-ones/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_moves(nums: Vec<i32>, k: i32) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1703() {
    }
}
