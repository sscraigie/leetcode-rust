/**
 * [1680] Concatenation of Consecutive Binary Numbers
 *
 * Given an integer n, return the decimal value of the binary string formed by concatenating the binary representations of 1 to n in order, modulo 10^9 + 7.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: n = 1
 * Output: 1
 * Explanation: "1" in binary corresponds to the decimal value 1. 
 * 
 * <strong class="example">Example 2:
 * 
 * Input: n = 3
 * Output: 27
 * Explanation: In binary, 1, 2, and 3 corresponds to "1", "10", and "11".
 * After concatenating them, we have "11011", which corresponds to the decimal value 27.
 * 
 * <strong class="example">Example 3:
 * 
 * Input: n = 12
 * Output: 505379714
 * Explanation: The concatenation results in "1101110010111011110001001101010111100".
 * The decimal value of that is 118505380540.
 * After modulo 10^9 + 7, the result is 505379714.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= n <= 10^5
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/concatenation-of-consecutive-binary-numbers/
// discuss: https://leetcode.com/problems/concatenation-of-consecutive-binary-numbers/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn concatenated_binary(n: i32) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1680() {
    }
}
