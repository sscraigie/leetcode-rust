/**
 * [472] Concatenated Words
 *
 * Given an array of strings words (without duplicates), return all the concatenated words in the given list of words.
 * A concatenated word is defined as a string that is comprised entirely of at least two shorter words (not necesssarily distinct) in the given array.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: words = ["cat","cats","catsdogcats","dog","dogcatsdog","hippopotamuses","rat","ratcatdogcat"]
 * Output: ["catsdogcats","dogcatsdog","ratcatdogcat"]
 * Explanation: "catsdogcats" can be concatenated by "cats", "dog" and "cats"; 
 * "dogcatsdog" can be concatenated by "dog", "cats" and "dog"; 
 * "ratcatdogcat" can be concatenated by "rat", "cat", "dog" and "cat".
 * <strong class="example">Example 2:
 * 
 * Input: words = ["cat","dog","catdog"]
 * Output: ["catdog"]
 * 
 *  
 * Constraints:
 * 
 * 	1 <= words.length <= 10^4
 * 	1 <= words[i].length <= 30
 * 	words[i] consists of only lowercase English letters.
 * 	All the strings of words are unique.
 * 	1 <= sum(words[i].length) <= 10^5
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/concatenated-words/
// discuss: https://leetcode.com/problems/concatenated-words/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_all_concatenated_words_in_a_dict(words: Vec<String>) -> Vec<String> {
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_472() {
    }
}
