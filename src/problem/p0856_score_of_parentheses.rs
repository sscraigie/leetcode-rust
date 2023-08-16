/**
 * [856] Score of Parentheses
 *
 * Given a balanced parentheses string s, return the score of the string.
 * The score of a balanced parentheses string is based on the following rule:
 * 
 * 	"()" has score 1.
 * 	AB has score A + B, where A and B are balanced parentheses strings.
 * 	(A) has score 2 * A, where A is a balanced parentheses string.
 * 
 *  
 * <strong class="example">Example 1:
 * 
 * Input: s = "()"
 * Output: 1
 * 
 * <strong class="example">Example 2:
 * 
 * Input: s = "(())"
 * Output: 2
 * 
 * <strong class="example">Example 3:
 * 
 * Input: s = "()()"
 * Output: 2
 * 
 *  
 * Constraints:
 * 
 * 	2 <= s.length <= 50
 * 	s consists of only '(' and ')'.
 * 	s is a balanced parentheses string.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/score-of-parentheses/
// discuss: https://leetcode.com/problems/score-of-parentheses/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_856() {
    }
}
