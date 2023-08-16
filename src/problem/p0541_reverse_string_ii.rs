/**
 * [541] Reverse String II
 *
 * Given a string s and an integer k, reverse the first k characters for every 2k characters counting from the start of the string.
 * If there are fewer than k characters left, reverse all of them. If there are less than 2k but greater than or equal to k characters, then reverse the first k characters and leave the other as original.
 *  
 * <strong class="example">Example 1:
 * Input: s = "abcdefg", k = 2
 * Output: "bacdfeg"
 * <strong class="example">Example 2:
 * Input: s = "abcd", k = 2
 * Output: "bacd"
 *  
 * Constraints:
 * 
 * 	1 <= s.length <= 10^4
 * 	s consists of only lowercase English letters.
 * 	1 <= k <= 10^4
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/reverse-string-ii/
// discuss: https://leetcode.com/problems/reverse-string-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        String::new()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_541() {
    }
}
