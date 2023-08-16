/**
 * [1545] Find Kth Bit in Nth Binary String
 *
 * Given two positive integers n and k, the binary string Sn is formed as follows:
 * 
 * 	S1 = "0"
 * 	Si = Si - 1 + "1" + reverse(invert(Si - 1)) for i > 1
 * 
 * Where + denotes the concatenation operation, reverse(x) returns the reversed string x, and invert(x) inverts all the bits in x (0 changes to 1 and 1 changes to 0).
 * For example, the first four strings in the above sequence are:
 * 
 * 	S1 = "0"
 * 	S2 = "011"
 * 	S3 = "0111001"
 * 	S4 = "011100110110001"
 * 
 * Return the k^th bit in Sn. It is guaranteed that k is valid for the given n.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: n = 3, k = 1
 * Output: "0"
 * Explanation: S3 is "<u>0</u>111001".
 * The 1^st bit is "0".
 * 
 * <strong class="example">Example 2:
 * 
 * Input: n = 4, k = 11
 * Output: "1"
 * Explanation: S4 is "0111001101<u>1</u>0001".
 * The 11^th bit is "1".
 * 
 *  
 * Constraints:
 * 
 * 	1 <= n <= 20
 * 	1 <= k <= 2^n - 1
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-kth-bit-in-nth-binary-string/
// discuss: https://leetcode.com/problems/find-kth-bit-in-nth-binary-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_kth_bit(n: i32, k: i32) -> char {
        '0'
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1545() {
    }
}
