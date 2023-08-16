/**
 * [915] Partition Array into Disjoint Intervals
 *
 * Given an integer array nums, partition it into two (contiguous) subarrays left and right so that:
 * 
 * 	Every element in left is less than or equal to every element in right.
 * 	left and right are non-empty.
 * 	left has the smallest possible size.
 * 
 * Return the length of left after such a partitioning.
 * Test cases are generated such that partitioning exists.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: nums = [5,0,3,8,6]
 * Output: 3
 * Explanation: left = [5,0,3], right = [8,6]
 * 
 * <strong class="example">Example 2:
 * 
 * Input: nums = [1,1,1,0,6,12]
 * Output: 4
 * Explanation: left = [1,1,1,0], right = [6,12]
 * 
 *  
 * Constraints:
 * 
 * 	2 <= nums.length <= 10^5
 * 	0 <= nums[i] <= 10^6
 * 	There is at least one valid answer for the given input.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/partition-array-into-disjoint-intervals/
// discuss: https://leetcode.com/problems/partition-array-into-disjoint-intervals/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn partition_disjoint(nums: Vec<i32>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_915() {
    }
}
