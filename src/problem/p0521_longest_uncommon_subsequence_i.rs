/**
 * [521] Longest Uncommon Subsequence I
 *
 * Given two strings a and b, return the length of the longest uncommon subsequence between a and b. If the longest uncommon subsequence does not exist, return -1.
 * An uncommon subsequence between two strings is a string that is a subsequence of one but not the other.
 * A subsequence of a string s is a string that can be obtained after deleting any number of characters from s.
 * 
 * 	For example, "abc" is a subsequence of "aebdc" because you can delete the underlined characters in "a<u>e</u>b<u>d</u>c" to get "abc". Other subsequences of "aebdc" include "aebdc", "aeb", and "" (empty string).
 * 
 *  
 * <strong class="example">Example 1:
 * 
 * Input: a = "aba", b = "cdc"
 * Output: 3
 * Explanation: One longest uncommon subsequence is "aba" because "aba" is a subsequence of "aba" but not "cdc".
 * Note that "cdc" is also a longest uncommon subsequence.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: a = "aaa", b = "bbb"
 * Output: 3
 * Explanation: The longest uncommon subsequences are "aaa" and "bbb".
 * 
 * <strong class="example">Example 3:
 * 
 * Input: a = "aaa", b = "aaa"
 * Output: -1
 * Explanation: Every subsequence of string a is also a subsequence of string b. Similarly, every subsequence of string b is also a subsequence of string a.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= a.length, b.length <= 100
 * 	a and b consist of lower-case English letters.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-uncommon-subsequence-i/
// discuss: https://leetcode.com/problems/longest-uncommon-subsequence-i/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_lu_slength(a: String, b: String) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_521() {
    }
}
