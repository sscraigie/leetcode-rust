/**
 * [2447] Number of Subarrays With GCD Equal to K
 *
 * Given an integer array nums and an integer k, return the number of subarrays of nums where the greatest common divisor of the subarray's elements is k.
 * A subarray is a contiguous non-empty sequence of elements within an array.
 * The greatest common divisor of an array is the largest integer that evenly divides all the array elements.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: nums = [9,3,1,2,6,3], k = 3
 * Output: 4
 * Explanation: The subarrays of nums where 3 is the greatest common divisor of all the subarray's elements are:
 * - [9,<u>3</u>,1,2,6,3]
 * - [9,3,1,2,6,<u>3</u>]
 * - [<u>9,3</u>,1,2,6,3]
 * - [9,3,1,2,<u>6,3</u>]
 * 
 * <strong class="example">Example 2:
 * 
 * Input: nums = [4], k = 7
 * Output: 0
 * Explanation: There are no subarrays of nums where 7 is the greatest common divisor of all the subarray's elements.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 1000
 * 	1 <= nums[i], k <= 10^9
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-subarrays-with-gcd-equal-to-k/
// discuss: https://leetcode.com/problems/number-of-subarrays-with-gcd-equal-to-k/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn subarray_gcd(nums: Vec<i32>, k: i32) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2447() {
    }
}
