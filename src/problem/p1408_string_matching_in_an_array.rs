/**
 * [1408] String Matching in an Array
 *
 * Given an array of string words, return all strings in words that is a substring of another word. You can return the answer in any order.
 * A substring is a contiguous sequence of characters within a string
 *  
 * <strong class="example">Example 1:
 * 
 * Input: words = ["mass","as","hero","superhero"]
 * Output: ["as","hero"]
 * Explanation: "as" is substring of "mass" and "hero" is substring of "superhero".
 * ["hero","as"] is also a valid answer.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: words = ["leetcode","et","code"]
 * Output: ["et","code"]
 * Explanation: "et", "code" are substring of "leetcode".
 * 
 * <strong class="example">Example 3:
 * 
 * Input: words = ["blue","green","bu"]
 * Output: []
 * Explanation: No string of words is substring of another string.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= words.length <= 100
 * 	1 <= words[i].length <= 30
 * 	words[i] contains only lowercase English letters.
 * 	All the strings of words are unique.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/string-matching-in-an-array/
// discuss: https://leetcode.com/problems/string-matching-in-an-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn string_matching(words: Vec<String>) -> Vec<String> {
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1408() {
    }
}
