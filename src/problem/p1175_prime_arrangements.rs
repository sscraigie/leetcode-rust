/**
 * [1175] Prime Arrangements
 *
 * Return the number of permutations of 1 to n so that prime numbers are at prime indices (1-indexed.)
 * (Recall that an integer is prime if and only if it is greater than 1, and cannot be written as a product of two positive integers both smaller than it.)
 * Since the answer may be large, return the answer modulo 10^9 + 7.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: n = 5
 * Output: 12
 * Explanation: For example [1,2,5,4,3] is a valid permutation, but [5,2,3,4,1] is not because the prime number 5 is at index 1.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: n = 100
 * Output: 682289015
 * 
 *  
 * Constraints:
 * 
 * 	1 <= n <= 100
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/prime-arrangements/
// discuss: https://leetcode.com/problems/prime-arrangements/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn num_prime_arrangements(n: i32) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1175() {
    }
}
