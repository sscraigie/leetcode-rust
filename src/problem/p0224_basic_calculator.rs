/**
 * [224] Basic Calculator
 *
 * Given a string s representing a valid expression, implement a basic calculator to evaluate it, and return the result of the evaluation.
 * Note: You are not allowed to use any built-in function which evaluates strings as mathematical expressions, such as eval().
 *  
 * <strong class="example">Example 1:
 * 
 * Input: s = "1 + 1"
 * Output: 2
 * 
 * <strong class="example">Example 2:
 * 
 * Input: s = " 2-1 + 2 "
 * Output: 3
 * 
 * <strong class="example">Example 3:
 * 
 * Input: s = "(1+(4+5+2)-3)+(6+8)"
 * Output: 23
 * 
 *  
 * Constraints:
 * 
 * 	1 <= s.length <= 3 * 10^5
 * 	s consists of digits, '+', '-', '(', ')', and ' '.
 * 	s represents a valid expression.
 * 	'+' is not used as a unary operation (i.e., "+1" and "+(2 + 3)" is invalid).
 * 	'-' could be used as a unary operation (i.e., "-1" and "-(2 + 3)" is valid).
 * 	There will be no two consecutive operators in the input.
 * 	Every number and running calculation will fit in a signed 32-bit integer.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/basic-calculator/
// discuss: https://leetcode.com/problems/basic-calculator/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn calculate(s: String) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_224() {
    }
}
