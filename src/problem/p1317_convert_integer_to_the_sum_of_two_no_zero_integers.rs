/**
 * [1317] Convert Integer to the Sum of Two No-Zero Integers
 *
 * No-Zero integer is a positive integer that does not contain any 0 in its decimal representation.
 * Given an integer n, return a list of two integers [a, b] where:
 * 
 * 	a and b are No-Zero integers.
 * 	a + b = n
 * 
 * The test cases are generated so that there is at least one valid solution. If there are many valid solutions, you can return any of them.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: n = 2
 * Output: [1,1]
 * Explanation: Let a = 1 and b = 1.
 * Both a and b are no-zero integers, and a + b = 2 = n.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: n = 11
 * Output: [2,9]
 * Explanation: Let a = 2 and b = 9.
 * Both a and b are no-zero integers, and a + b = 9 = n.
 * Note that there are other valid answers as [8, 3] that can be accepted.
 * 
 *  
 * Constraints:
 * 
 * 	2 <= n <= 10^4
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/convert-integer-to-the-sum-of-two-no-zero-integers/
// discuss: https://leetcode.com/problems/convert-integer-to-the-sum-of-two-no-zero-integers/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn get_no_zero_integers(n: i32) -> Vec<i32> {
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1317() {
    }
}
