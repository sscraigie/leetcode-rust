/**
 * [1234] Replace the Substring for Balanced String
 *
 * You are given a string s of length n containing only four kinds of characters: 'Q', 'W', 'E', and 'R'.
 * A string is said to be balanced if each of its characters appears n / 4 times where n is the length of the string.
 * Return the minimum length of the substring that can be replaced with any other string of the same length to make s balanced. If s is already balanced, return 0.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: s = "QWER"
 * Output: 0
 * Explanation: s is already balanced.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: s = "QQWE"
 * Output: 1
 * Explanation: We need to replace a 'Q' to 'R', so that "RQWE" (or "QRWE") is balanced.
 * 
 * <strong class="example">Example 3:
 * 
 * Input: s = "QQQW"
 * Output: 2
 * Explanation: We can replace the first "QQ" to "ER". 
 * 
 *  
 * Constraints:
 * 
 * 	n == s.length
 * 	4 <= n <= 10^5
 * 	n is a multiple of 4.
 * 	s contains only 'Q', 'W', 'E', and 'R'.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/replace-the-substring-for-balanced-string/
// discuss: https://leetcode.com/problems/replace-the-substring-for-balanced-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn balanced_string(s: String) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1234() {
    }
}
