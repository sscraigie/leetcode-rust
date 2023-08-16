/**
 * [1190] Reverse Substrings Between Each Pair of Parentheses
 *
 * You are given a string s that consists of lower case English letters and brackets.
 * Reverse the strings in each pair of matching parentheses, starting from the innermost one.
 * Your result should not contain any brackets.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: s = "(abcd)"
 * Output: "dcba"
 * 
 * <strong class="example">Example 2:
 * 
 * Input: s = "(u(love)i)"
 * Output: "iloveu"
 * Explanation: The substring "love" is reversed first, then the whole string is reversed.
 * 
 * <strong class="example">Example 3:
 * 
 * Input: s = "(ed(et(oc))el)"
 * Output: "leetcode"
 * Explanation: First, we reverse the substring "oc", then "etco", and finally, the whole string.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= s.length <= 2000
 * 	s only contains lower case English characters and parentheses.
 * 	It is guaranteed that all parentheses are balanced.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/reverse-substrings-between-each-pair-of-parentheses/
// discuss: https://leetcode.com/problems/reverse-substrings-between-each-pair-of-parentheses/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn reverse_parentheses(s: String) -> String {
        String::new()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1190() {
    }
}
