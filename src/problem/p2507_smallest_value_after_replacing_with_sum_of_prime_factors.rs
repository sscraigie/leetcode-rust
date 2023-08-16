/**
 * [2507] Smallest Value After Replacing With Sum of Prime Factors
 *
 * You are given a positive integer n.
 * Continuously replace n with the sum of its prime factors.
 * 
 * 	Note that if a prime factor divides n multiple times, it should be included in the sum as many times as it divides n.
 * 
 * Return the smallest value n will take on.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: n = 15
 * Output: 5
 * Explanation: Initially, n = 15.
 * 15 = 3 * 5, so replace n with 3 + 5 = 8.
 * 8 = 2 * 2 * 2, so replace n with 2 + 2 + 2 = 6.
 * 6 = 2 * 3, so replace n with 2 + 3 = 5.
 * 5 is the smallest value n will take on.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: n = 3
 * Output: 3
 * Explanation: Initially, n = 3.
 * 3 is the smallest value n will take on.
 * 
 *  
 * Constraints:
 * 
 * 	2 <= n <= 10^5
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/smallest-value-after-replacing-with-sum-of-prime-factors/
// discuss: https://leetcode.com/problems/smallest-value-after-replacing-with-sum-of-prime-factors/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn smallest_value(n: i32) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2507() {
    }
}
