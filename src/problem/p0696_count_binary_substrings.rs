/**
 * [696] Count Binary Substrings
 *
 * Given a binary string s, return the number of non-empty substrings that have the same number of 0's and 1's, and all the 0's and all the 1's in these substrings are grouped consecutively.
 * Substrings that occur multiple times are counted the number of times they occur.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: s = "00110011"
 * Output: 6
 * Explanation: There are 6 substrings that have equal number of consecutive 1's and 0's: "0011", "01", "1100", "10", "0011", and "01".
 * Notice that some of these substrings repeat and are counted the number of times they occur.
 * Also, "00110011" is not a valid substring because all the 0's (and 1's) are not grouped together.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: s = "10101"
 * Output: 4
 * Explanation: There are 4 substrings: "10", "01", "10", "01" that have equal number of consecutive 1's and 0's.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= s.length <= 10^5
 * 	s[i] is either '0' or '1'.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-binary-substrings/
// discuss: https://leetcode.com/problems/count-binary-substrings/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_binary_substrings(s: String) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_696() {
    }
}
