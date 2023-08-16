/**
 * [629] K Inverse Pairs Array
 *
 * For an integer array nums, an inverse pair is a pair of integers [i, j] where 0 <= i < j < nums.length and nums[i] > nums[j].
 * Given two integers n and k, return the number of different arrays consist of numbers from 1 to n such that there are exactly k inverse pairs. Since the answer can be huge, return it modulo 10^9 + 7.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: n = 3, k = 0
 * Output: 1
 * Explanation: Only the array [1,2,3] which consists of numbers from 1 to 3 has exactly 0 inverse pairs.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: n = 3, k = 1
 * Output: 2
 * Explanation: The array [1,3,2] and [2,1,3] have exactly 1 inverse pair.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= n <= 1000
 * 	0 <= k <= 1000
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/k-inverse-pairs-array/
// discuss: https://leetcode.com/problems/k-inverse-pairs-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn k_inverse_pairs(n: i32, k: i32) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_629() {
    }
}
