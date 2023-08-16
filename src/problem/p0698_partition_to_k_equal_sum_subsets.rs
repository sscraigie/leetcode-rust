/**
 * [698] Partition to K Equal Sum Subsets
 *
 * Given an integer array nums and an integer k, return true if it is possible to divide this array into k non-empty subsets whose sums are all equal.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: nums = [4,3,2,3,5,2,1], k = 4
 * Output: true
 * Explanation: It is possible to divide it into 4 subsets (5), (1, 4), (2,3), (2,3) with equal sums.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: nums = [1,2,3,4], k = 3
 * Output: false
 * 
 *  
 * Constraints:
 * 
 * 	1 <= k <= nums.length <= 16
 * 	1 <= nums[i] <= 10^4
 * 	The frequency of each element is in the range [1, 4].
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/partition-to-k-equal-sum-subsets/
// discuss: https://leetcode.com/problems/partition-to-k-equal-sum-subsets/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool {
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_698() {
    }
}
