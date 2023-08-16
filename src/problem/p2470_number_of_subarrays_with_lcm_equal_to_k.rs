/**
 * [2470] Number of Subarrays With LCM Equal to K
 *
 * Given an integer array nums and an integer k, return the number of subarrays of nums where the least common multiple of the subarray's elements is k.
 * A subarray is a contiguous non-empty sequence of elements within an array.
 * The least common multiple of an array is the smallest positive integer that is divisible by all the array elements.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: nums = [3,6,2,7,1], k = 6
 * Output: 4
 * Explanation: The subarrays of nums where 6 is the least common multiple of all the subarray's elements are:
 * - [<u>3</u>,<u>6</u>,2,7,1]
 * - [<u>3</u>,<u>6</u>,<u>2</u>,7,1]
 * - [3,<u>6</u>,2,7,1]
 * - [3,<u>6</u>,<u>2</u>,7,1]
 * 
 * <strong class="example">Example 2:
 * 
 * Input: nums = [3], k = 2
 * Output: 0
 * Explanation: There are no subarrays of nums where 2 is the least common multiple of all the subarray's elements.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 1000
 * 	1 <= nums[i], k <= 1000
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-subarrays-with-lcm-equal-to-k/
// discuss: https://leetcode.com/problems/number-of-subarrays-with-lcm-equal-to-k/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn subarray_lcm(nums: Vec<i32>, k: i32) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2470() {
    }
}
