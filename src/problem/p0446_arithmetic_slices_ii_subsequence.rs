/**
 * [446] Arithmetic Slices II - Subsequence
 *
 * Given an integer array nums, return the number of all the arithmetic subsequences of nums.
 * A sequence of numbers is called arithmetic if it consists of at least three elements and if the difference between any two consecutive elements is the same.
 * 
 * 	For example, [1, 3, 5, 7, 9], [7, 7, 7, 7], and [3, -1, -5, -9] are arithmetic sequences.
 * 	For example, [1, 1, 2, 5, 7] is not an arithmetic sequence.
 * 
 * A subsequence of an array is a sequence that can be formed by removing some elements (possibly none) of the array.
 * 
 * 	For example, [2,5,10] is a subsequence of [1,2,1,<u>2</u>,4,1,<u>5</u>,<u>10</u>].
 * 
 * The test cases are generated so that the answer fits in 32-bit integer.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: nums = [2,4,6,8,10]
 * Output: 7
 * Explanation: All arithmetic subsequence slices are:
 * [2,4,6]
 * [4,6,8]
 * [6,8,10]
 * [2,4,6,8]
 * [4,6,8,10]
 * [2,4,6,8,10]
 * [2,6,10]
 * 
 * <strong class="example">Example 2:
 * 
 * Input: nums = [7,7,7,7,7]
 * Output: 16
 * Explanation: Any subsequence of this array is arithmetic.
 * 
 *  
 * Constraints:
 * 
 * 	1  <= nums.length <= 1000
 * 	-2^31 <= nums[i] <= 2^31 - 1
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/arithmetic-slices-ii-subsequence/
// discuss: https://leetcode.com/problems/arithmetic-slices-ii-subsequence/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_446() {
    }
}
