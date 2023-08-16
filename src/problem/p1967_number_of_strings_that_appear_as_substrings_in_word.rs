/**
 * [1967] Number of Strings That Appear as Substrings in Word
 *
 * Given an array of strings patterns and a string word, return the number of strings in patterns that exist as a substring in word.
 * A substring is a contiguous sequence of characters within a string.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: patterns = ["a","abc","bc","d"], word = "abc"
 * Output: 3
 * Explanation:
 * - "a" appears as a substring in "<u>a</u>bc".
 * - "abc" appears as a substring in "<u>abc</u>".
 * - "bc" appears as a substring in "a<u>bc</u>".
 * - "d" does not appear as a substring in "abc".
 * 3 of the strings in patterns appear as a substring in word.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: patterns = ["a","b","c"], word = "aaaaabbbbb"
 * Output: 2
 * Explanation:
 * - "a" appears as a substring in "a<u>a</u>aaabbbbb".
 * - "b" appears as a substring in "aaaaabbbb<u>b</u>".
 * - "c" does not appear as a substring in "aaaaabbbbb".
 * 2 of the strings in patterns appear as a substring in word.
 * 
 * <strong class="example">Example 3:
 * 
 * Input: patterns = ["a","a","a"], word = "ab"
 * Output: 3
 * Explanation: Each of the patterns appears as a substring in word "<u>a</u>b".
 * 
 *  
 * Constraints:
 * 
 * 	1 <= patterns.length <= 100
 * 	1 <= patterns[i].length <= 100
 * 	1 <= word.length <= 100
 * 	patterns[i] and word consist of lowercase English letters.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-strings-that-appear-as-substrings-in-word/
// discuss: https://leetcode.com/problems/number-of-strings-that-appear-as-substrings-in-word/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn num_of_strings(patterns: Vec<String>, word: String) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1967() {
    }
}
