/**
 * [467] Unique Substrings in Wraparound String
 *
 * We define the string base to be the infinite wraparound string of "abcdefghijklmnopqrstuvwxyz", so base will look like this:
 * 
 * 	"...zabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcd....".
 * 
 * Given a string s, return the number of unique non-empty substrings of s are present in base.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: s = "a"
 * Output: 1
 * Explanation: Only the substring "a" of s is in base.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: s = "cac"
 * Output: 2
 * Explanation: There are two substrings ("a", "c") of s in base.
 * 
 * <strong class="example">Example 3:
 * 
 * Input: s = "zab"
 * Output: 6
 * Explanation: There are six substrings ("z", "a", "b", "za", "ab", and "zab") of s in base.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= s.length <= 10^5
 * 	s consists of lowercase English letters.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/unique-substrings-in-wraparound-string/
// discuss: https://leetcode.com/problems/unique-substrings-in-wraparound-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_substring_in_wrapround_string(s: String) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_467() {
    }
}
