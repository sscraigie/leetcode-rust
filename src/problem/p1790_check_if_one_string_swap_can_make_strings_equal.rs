/**
 * [1790] Check if One String Swap Can Make Strings Equal
 *
 * You are given two strings s1 and s2 of equal length. A string swap is an operation where you choose two indices in a string (not necessarily different) and swap the characters at these indices.
 * Return true if it is possible to make both strings equal by performing at most one string swap on exactly one of the strings. Otherwise, return false.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: s1 = "bank", s2 = "kanb"
 * Output: true
 * Explanation: For example, swap the first character with the last character of s2 to make "bank".
 * 
 * <strong class="example">Example 2:
 * 
 * Input: s1 = "attack", s2 = "defend"
 * Output: false
 * Explanation: It is impossible to make them equal with one string swap.
 * 
 * <strong class="example">Example 3:
 * 
 * Input: s1 = "kelb", s2 = "kelb"
 * Output: true
 * Explanation: The two strings are already equal, so no string swap operation is required.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= s1.length, s2.length <= 100
 * 	s1.length == s2.length
 * 	s1 and s2 consist of only lowercase English letters.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/check-if-one-string-swap-can-make-strings-equal/
// discuss: https://leetcode.com/problems/check-if-one-string-swap-can-make-strings-equal/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1790() {
    }
}
