/**
 * [76] Minimum Window Substring
 *
 * Given two strings s and t of lengths m and n respectively, return the minimum window <span data-keyword="substring-nonempty">substring</span> of s such that every character in t (including duplicates) is included in the window. If there is no such substring, return the empty string "".
 * The testcases will be generated such that the answer is unique.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: s = "ADOBECODEBANC", t = "ABC"
 * Output: "BANC"
 * Explanation: The minimum window substring "BANC" includes 'A', 'B', and 'C' from string t.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: s = "a", t = "a"
 * Output: "a"
 * Explanation: The entire string s is the minimum window.
 * 
 * <strong class="example">Example 3:
 * 
 * Input: s = "a", t = "aa"
 * Output: ""
 * Explanation: Both 'a's from t must be included in the window.
 * Since the largest window of s only has one 'a', return empty string.
 * 
 *  
 * Constraints:
 * 
 * 	m == s.length
 * 	n == t.length
 * 	1 <= m, n <= 10^5
 * 	s and t consist of uppercase and lowercase English letters.
 * 
 *  
 * Follow up: Could you find an algorithm that runs in O(m + n) time?
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-window-substring/
// discuss: https://leetcode.com/problems/minimum-window-substring/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        String::new()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_76() {
    }
}
