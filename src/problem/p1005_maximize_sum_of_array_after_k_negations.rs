/**
 * [1005] Maximize Sum Of Array After K Negations
 *
 * Given an integer array nums and an integer k, modify the array in the following way:
 * 
 * 	choose an index i and replace nums[i] with -nums[i].
 * 
 * You should apply this process exactly k times. You may choose the same index i multiple times.
 * Return the largest possible sum of the array after modifying it in this way.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: nums = [4,2,3], k = 1
 * Output: 5
 * Explanation: Choose index 1 and nums becomes [4,-2,3].
 * 
 * <strong class="example">Example 2:
 * 
 * Input: nums = [3,-1,0,2], k = 3
 * Output: 6
 * Explanation: Choose indices (1, 2, 2) and nums becomes [3,1,0,2].
 * 
 * <strong class="example">Example 3:
 * 
 * Input: nums = [2,-3,-1,5,-4], k = 2
 * Output: 13
 * Explanation: Choose indices (1, 4) and nums becomes [2,3,-1,5,4].
 * 
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 10^4
 * 	-100 <= nums[i] <= 100
 * 	1 <= k <= 10^4
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximize-sum-of-array-after-k-negations/
// discuss: https://leetcode.com/problems/maximize-sum-of-array-after-k-negations/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn largest_sum_after_k_negations(nums: Vec<i32>, k: i32) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1005() {
    }
}
