/**
 * [1160] Find Words That Can Be Formed by Characters
 *
 * You are given an array of strings words and a string chars.
 * A string is good if it can be formed by characters from chars (each character can only be used once).
 * Return the sum of lengths of all good strings in words.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: words = ["cat","bt","hat","tree"], chars = "atach"
 * Output: 6
 * Explanation: The strings that can be formed are "cat" and "hat" so the answer is 3 + 3 = 6.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: words = ["hello","world","leetcode"], chars = "welldonehoneyr"
 * Output: 10
 * Explanation: The strings that can be formed are "hello" and "world" so the answer is 5 + 5 = 10.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= words.length <= 1000
 * 	1 <= words[i].length, chars.length <= 100
 * 	words[i] and chars consist of lowercase English letters.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-words-that-can-be-formed-by-characters/
// discuss: https://leetcode.com/problems/find-words-that-can-be-formed-by-characters/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1160() {
    }
}
