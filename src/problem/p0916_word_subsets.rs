/**
 * [916] Word Subsets
 *
 * You are given two string arrays words1 and words2.
 * A string b is a subset of string a if every letter in b occurs in a including multiplicity.
 * 
 * 	For example, "wrr" is a subset of "warrior" but is not a subset of "world".
 * 
 * A string a from words1 is universal if for every string b in words2, b is a subset of a.
 * Return an array of all the universal strings in words1. You may return the answer in any order.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: words1 = ["amazon","apple","facebook","google","leetcode"], words2 = ["e","o"]
 * Output: ["facebook","google","leetcode"]
 * 
 * <strong class="example">Example 2:
 * 
 * Input: words1 = ["amazon","apple","facebook","google","leetcode"], words2 = ["l","e"]
 * Output: ["apple","google","leetcode"]
 * 
 *  
 * Constraints:
 * 
 * 	1 <= words1.length, words2.length <= 10^4
 * 	1 <= words1[i].length, words2[i].length <= 10
 * 	words1[i] and words2[i] consist only of lowercase English letters.
 * 	All the strings of words1 are unique.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/word-subsets/
// discuss: https://leetcode.com/problems/word-subsets/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_916() {
    }
}
