/**
 * [2750] Ways to Split Array Into Good Subarrays
 *
 * You are given a binary array nums.
 * A subarray of an array is good if it contains exactly one element with the value 1.
 * Return an integer denoting the number of ways to split the array nums into good subarrays. As the number may be too large, return it modulo 10^9 + 7.
 * A subarray is a contiguous non-empty sequence of elements within an array.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: nums = [0,1,0,0,1]
 * Output: 3
 * Explanation: There are 3 ways to split nums into good subarrays:
 * - [0,1] [0,0,1]
 * - [0,1,0] [0,1]
 * - [0,1,0,0] [1]
 * 
 * <strong class="example">Example 2:
 * 
 * Input: nums = [0,1,0]
 * Output: 1
 * Explanation: There is 1 way to split nums into good subarrays:
 * - [0,1,0]
 * 
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 10^5
 * 	0 <= nums[i] <= 1
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/ways-to-split-array-into-good-subarrays/
// discuss: https://leetcode.com/problems/ways-to-split-array-into-good-subarrays/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn number_of_good_subarray_splits(nums: Vec<i32>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2750() {
    }
}
