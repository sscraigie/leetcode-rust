/**
 * [1392] Longest Happy Prefix
 *
 * A string is called a happy prefix if is a non-empty prefix which is also a suffix (excluding itself).
 * Given a string s, return the longest happy prefix of s. Return an empty string "" if no such prefix exists.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: s = "level"
 * Output: "l"
 * Explanation: s contains 4 prefix excluding itself ("l", "le", "lev", "leve"), and suffix ("l", "el", "vel", "evel"). The largest prefix which is also suffix is given by "l".
 * 
 * <strong class="example">Example 2:
 * 
 * Input: s = "ababab"
 * Output: "abab"
 * Explanation: "abab" is the largest prefix which is also suffix. They can overlap in the original string.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= s.length <= 10^5
 * 	s contains only lowercase English letters.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-happy-prefix/
// discuss: https://leetcode.com/problems/longest-happy-prefix/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn longest_prefix(s: String) -> String {
        String::new()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1392() {
    }
}
