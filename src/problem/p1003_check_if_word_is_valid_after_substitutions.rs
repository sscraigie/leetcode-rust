/**
 * [1003] Check If Word Is Valid After Substitutions
 *
 * Given a string s, determine if it is valid.
 * A string s is valid if, starting with an empty string t = "", you can transform t into s after performing the following operation any number of times:
 * 
 * 	Insert string "abc" into any position in t. More formally, t becomes tleft + "abc" + tright, where t == tleft + tright. Note that tleft and tright may be empty.
 * 
 * Return true if s is a valid string, otherwise, return false.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: s = "aabcbc"
 * Output: true
 * Explanation:
 * "" -> "<u>abc</u>" -> "a<u>abc</u>bc"
 * Thus, "aabcbc" is valid.
 * <strong class="example">Example 2:
 * 
 * Input: s = "abcabcababcc"
 * Output: true
 * Explanation:
 * "" -> "<u>abc</u>" -> "abc<u>abc</u>" -> "abcabc<u>abc</u>" -> "abcabcab<u>abc</u>c"
 * Thus, "abcabcababcc" is valid.
 * 
 * <strong class="example">Example 3:
 * 
 * Input: s = "abccba"
 * Output: false
 * Explanation: It is impossible to get "abccba" using the operation.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= s.length <= 2 * 10^4
 * 	s consists of letters 'a', 'b', and 'c'
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/check-if-word-is-valid-after-substitutions/
// discuss: https://leetcode.com/problems/check-if-word-is-valid-after-substitutions/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_valid(s: String) -> bool {
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1003() {
    }
}
