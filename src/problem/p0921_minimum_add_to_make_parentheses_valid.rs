/**
 * [921] Minimum Add to Make Parentheses Valid
 *
 * A parentheses string is valid if and only if:
 * 
 * 	It is the empty string,
 * 	It can be written as AB (A concatenated with B), where A and B are valid strings, or
 * 	It can be written as (A), where A is a valid string.
 * 
 * You are given a parentheses string s. In one move, you can insert a parenthesis at any position of the string.
 * 
 * 	For example, if s = "()))", you can insert an opening parenthesis to be "(()))" or a closing parenthesis to be "())))".
 * 
 * Return the minimum number of moves required to make s valid.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: s = "())"
 * Output: 1
 * 
 * <strong class="example">Example 2:
 * 
 * Input: s = "((("
 * Output: 3
 * 
 *  
 * Constraints:
 * 
 * 	1 <= s.length <= 1000
 * 	s[i] is either '(' or ')'.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-add-to-make-parentheses-valid/
// discuss: https://leetcode.com/problems/minimum-add-to-make-parentheses-valid/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_921() {
    }
}
