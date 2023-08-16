/**
 * [65] Valid Number
 *
 * A valid number can be split up into these components (in order):
 * <ol>
 * 	A decimal number or an integer.
 * 	(Optional) An 'e' or 'E', followed by an integer.
 * </ol>
 * A decimal number can be split up into these components (in order):
 * <ol>
 * 	(Optional) A sign character (either '+' or '-').
 * 	One of the following formats:
 * 	<ol>
 * 		One or more digits, followed by a dot '.'.
 * 		One or more digits, followed by a dot '.', followed by one or more digits.
 * 		A dot '.', followed by one or more digits.
 * 	</ol>
 * 	
 * </ol>
 * An integer can be split up into these components (in order):
 * <ol>
 * 	(Optional) A sign character (either '+' or '-').
 * 	One or more digits.
 * </ol>
 * For example, all the following are valid numbers: ["2", "0089", "-0.1", "+3.14", "4.", "-.9", "2e10", "-90E3", "3e+7", "+6e-1", "53.5e93", "-123.456e789"], while the following are not valid numbers: ["abc", "1a", "1e", "e3", "99e2.5", "--6", "-+3", "95a54e53"].
 * Given a string s, return true if s is a valid number.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: s = "0"
 * Output: true
 * 
 * <strong class="example">Example 2:
 * 
 * Input: s = "e"
 * Output: false
 * 
 * <strong class="example">Example 3:
 * 
 * Input: s = "."
 * Output: false
 * 
 *  
 * Constraints:
 * 
 * 	1 <= s.length <= 20
 * 	s consists of only English letters (both uppercase and lowercase), digits (0-9), plus '+', minus '-', or dot '.'.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/valid-number/
// discuss: https://leetcode.com/problems/valid-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_number(s: String) -> bool {
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_65() {
    }
}
