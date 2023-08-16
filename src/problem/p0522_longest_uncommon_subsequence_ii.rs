/**
 * [522] Longest Uncommon Subsequence II
 *
 * Given an array of strings strs, return the length of the longest uncommon subsequence between them. If the longest uncommon subsequence does not exist, return -1.
 * An uncommon subsequence between an array of strings is a string that is a subsequence of one string but not the others.
 * A subsequence of a string s is a string that can be obtained after deleting any number of characters from s.
 * 
 * 	For example, "abc" is a subsequence of "aebdc" because you can delete the underlined characters in "a<u>e</u>b<u>d</u>c" to get "abc". Other subsequences of "aebdc" include "aebdc", "aeb", and "" (empty string).
 * 
 *  
 * <strong class="example">Example 1:
 * Input: strs = ["aba","cdc","eae"]
 * Output: 3
 * <strong class="example">Example 2:
 * Input: strs = ["aaa","aaa","aa"]
 * Output: -1
 *  
 * Constraints:
 * 
 * 	2 <= strs.length <= 50
 * 	1 <= strs[i].length <= 10
 * 	strs[i] consists of lowercase English letters.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-uncommon-subsequence-ii/
// discuss: https://leetcode.com/problems/longest-uncommon-subsequence-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_lu_slength(strs: Vec<String>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_522() {
    }
}
