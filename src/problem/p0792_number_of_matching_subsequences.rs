/**
 * [792] Number of Matching Subsequences
 *
 * Given a string s and an array of strings words, return the number of words[i] that is a subsequence of s.
 * A subsequence of a string is a new string generated from the original string with some characters (can be none) deleted without changing the relative order of the remaining characters.
 * 
 * 	For example, "ace" is a subsequence of "abcde".
 * 
 *  
 * <strong class="example">Example 1:
 * 
 * Input: s = "abcde", words = ["a","bb","acd","ace"]
 * Output: 3
 * Explanation: There are three strings in words that are a subsequence of s: "a", "acd", "ace".
 * 
 * <strong class="example">Example 2:
 * 
 * Input: s = "dsahjpjauf", words = ["ahjpjau","ja","ahbwzgqnuk","tnmlanowax"]
 * Output: 2
 * 
 *  
 * Constraints:
 * 
 * 	1 <= s.length <= 5 * 10^4
 * 	1 <= words.length <= 5000
 * 	1 <= words[i].length <= 50
 * 	s and words[i] consist of only lowercase English letters.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-matching-subsequences/
// discuss: https://leetcode.com/problems/number-of-matching-subsequences/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_792() {
    }
}
