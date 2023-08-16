/**
 * [140] Word Break II
 *
 * Given a string s and a dictionary of strings wordDict, add spaces in s to construct a sentence where each word is a valid dictionary word. Return all such possible sentences in any order.
 * Note that the same word in the dictionary may be reused multiple times in the segmentation.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: s = "catsanddog", wordDict = ["cat","cats","and","sand","dog"]
 * Output: ["cats and dog","cat sand dog"]
 * 
 * <strong class="example">Example 2:
 * 
 * Input: s = "pineapplepenapple", wordDict = ["apple","pen","applepen","pine","pineapple"]
 * Output: ["pine apple pen apple","pineapple pen apple","pine applepen apple"]
 * Explanation: Note that you are allowed to reuse a dictionary word.
 * 
 * <strong class="example">Example 3:
 * 
 * Input: s = "catsandog", wordDict = ["cats","dog","sand","and","cat"]
 * Output: []
 * 
 *  
 * Constraints:
 * 
 * 	1 <= s.length <= 20
 * 	1 <= wordDict.length <= 1000
 * 	1 <= wordDict[i].length <= 10
 * 	s and wordDict[i] consist of only lowercase English letters.
 * 	All the strings of wordDict are unique.
 * 	Input is generated in a way that the length of the answer doesn't exceed 10^5.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/word-break-ii/
// discuss: https://leetcode.com/problems/word-break-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_140() {
    }
}
