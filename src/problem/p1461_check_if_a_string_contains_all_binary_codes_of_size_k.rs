/**
 * [1461] Check If a String Contains All Binary Codes of Size K
 *
 * Given a binary string s and an integer k, return true if every binary code of length k is a substring of s. Otherwise, return false.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: s = "00110110", k = 2
 * Output: true
 * Explanation: The binary codes of length 2 are "00", "01", "10" and "11". They can be all found as substrings at indices 0, 1, 3 and 2 respectively.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: s = "0110", k = 1
 * Output: true
 * Explanation: The binary codes of length 1 are "0" and "1", it is clear that both exist as a substring. 
 * 
 * <strong class="example">Example 3:
 * 
 * Input: s = "0110", k = 2
 * Output: false
 * Explanation: The binary code "00" is of length 2 and does not exist in the array.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= s.length <= 5 * 10^5
 * 	s[i] is either '0' or '1'.
 * 	1 <= k <= 20
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/check-if-a-string-contains-all-binary-codes-of-size-k/
// discuss: https://leetcode.com/problems/check-if-a-string-contains-all-binary-codes-of-size-k/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1461() {
    }
}
