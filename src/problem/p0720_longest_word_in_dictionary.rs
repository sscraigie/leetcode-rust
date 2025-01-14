/**
 * [720] Longest Word in Dictionary
 *
 * Given an array of strings words representing an English Dictionary, return the longest word in words that can be built one character at a time by other words in words.
 * If there is more than one possible answer, return the longest word with the smallest lexicographical order. If there is no answer, return the empty string.
 * Note that the word should be built from left to right with each additional character being added to the end of a previous word. 
 *  
 * <strong class="example">Example 1:
 * 
 * Input: words = ["w","wo","wor","worl","world"]
 * Output: "world"
 * Explanation: The word "world" can be built one character at a time by "w", "wo", "wor", and "worl".
 * 
 * <strong class="example">Example 2:
 * 
 * Input: words = ["a","banana","app","appl","ap","apply","apple"]
 * Output: "apple"
 * Explanation: Both "apply" and "apple" can be built from other words in the dictionary. However, "apple" is lexicographically smaller than "apply".
 * 
 *  
 * Constraints:
 * 
 * 	1 <= words.length <= 1000
 * 	1 <= words[i].length <= 30
 * 	words[i] consists of lowercase English letters.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-word-in-dictionary/
// discuss: https://leetcode.com/problems/longest-word-in-dictionary/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn longest_word(words: Vec<String>) -> String {
        String::new()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_720() {
    }
}
