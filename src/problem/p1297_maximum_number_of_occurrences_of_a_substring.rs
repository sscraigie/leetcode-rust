/**
 * [1297] Maximum Number of Occurrences of a Substring
 *
 * Given a string s, return the maximum number of occurrences of any substring under the following rules:
 * 
 * 	The number of unique characters in the substring must be less than or equal to maxLetters.
 * 	The substring size must be between minSize and maxSize inclusive.
 * 
 *  
 * <strong class="example">Example 1:
 * 
 * Input: s = "aababcaab", maxLetters = 2, minSize = 3, maxSize = 4
 * Output: 2
 * Explanation: Substring "aab" has 2 occurrences in the original string.
 * It satisfies the conditions, 2 unique letters and size 3 (between minSize and maxSize).
 * 
 * <strong class="example">Example 2:
 * 
 * Input: s = "aaaa", maxLetters = 1, minSize = 3, maxSize = 3
 * Output: 2
 * Explanation: Substring "aaa" occur 2 times in the string. It can overlap.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= s.length <= 10^5
 * 	1 <= maxLetters <= 26
 * 	1 <= minSize <= maxSize <= min(26, s.length)
 * 	s consists of only lowercase English letters.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-number-of-occurrences-of-a-substring/
// discuss: https://leetcode.com/problems/maximum-number-of-occurrences-of-a-substring/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_freq(s: String, max_letters: i32, min_size: i32, max_size: i32) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1297() {
    }
}
