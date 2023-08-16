/**
 * [664] Strange Printer
 *
 * There is a strange printer with the following two special properties:
 * 
 * 	The printer can only print a sequence of the same character each time.
 * 	At each turn, the printer can print new characters starting from and ending at any place and will cover the original existing characters.
 * 
 * Given a string s, return the minimum number of turns the printer needed to print it.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: s = "aaabbb"
 * Output: 2
 * Explanation: Print "aaa" first and then print "bbb".
 * 
 * <strong class="example">Example 2:
 * 
 * Input: s = "aba"
 * Output: 2
 * Explanation: Print "aaa" first and then print "b" from the second place of the string, which will cover the existing character 'a'.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= s.length <= 100
 * 	s consists of lowercase English letters.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/strange-printer/
// discuss: https://leetcode.com/problems/strange-printer/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn strange_printer(s: String) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_664() {
    }
}
