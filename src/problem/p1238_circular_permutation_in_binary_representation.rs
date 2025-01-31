/**
 * [1238] Circular Permutation in Binary Representation
 *
 * Given 2 integers n and start. Your task is return any permutation p of (0,1,2.....,2^n -1) such that :
 * 
 * 
 * 	p[0] = start
 * 	p[i] and p[i+1] differ by only one bit in their binary representation.
 * 	p[0] and p[2^n -1] must also differ by only one bit in their binary representation.
 * 
 * 
 *  
 * <strong class="example">Example 1:
 * 
 * 
 * Input: n = 2, start = 3
 * Output: [3,2,0,1]
 * Explanation: The binary representation of the permutation is (11,10,00,01). 
 * All the adjacent element differ by one bit. Another valid permutation is [3,1,0,2]
 * 
 * 
 * <strong class="example">Example 2:
 * 
 * 
 * Input: n = 3, start = 2
 * Output: [2,6,7,5,4,0,1,3]
 * Explanation: The binary representation of the permutation is (010,110,111,101,100,000,001,011).
 * 
 * 
 *  
 * Constraints:
 * 
 * 
 * 	1 <= n <= 16
 * 	0 <= start < 2 ^ n
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/circular-permutation-in-binary-representation/
// discuss: https://leetcode.com/problems/circular-permutation-in-binary-representation/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn circular_permutation(n: i32, start: i32) -> Vec<i32> {
        
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1238() {
    }
}
