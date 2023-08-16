/**
 * [1668] Maximum Repeating Substring
 *
 * For a string sequence, a string word is k-repeating if word concatenated k times is a substring of sequence. The word's maximum k-repeating value is the highest value k where word is k-repeating in sequence. If word is not a substring of sequence, word's maximum k-repeating value is 0.
 * Given strings sequence and word, return the maximum k-repeating value of word in sequence.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: sequence = "ababc", word = "ab"
 * Output: 2
 * Explanation: "abab" is a substring in "<u>abab</u>c".
 * 
 * <strong class="example">Example 2:
 * 
 * Input: sequence = "ababc", word = "ba"
 * Output: 1
 * Explanation: "ba" is a substring in "a<u>ba</u>bc". "baba" is not a substring in "ababc".
 * 
 * <strong class="example">Example 3:
 * 
 * Input: sequence = "ababc", word = "ac"
 * Output: 0
 * Explanation: "ac" is not a substring in "ababc". 
 * 
 *  
 * Constraints:
 * 
 * 	1 <= sequence.length <= 100
 * 	1 <= word.length <= 100
 * 	sequence and word contains only lowercase English letters.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-repeating-substring/
// discuss: https://leetcode.com/problems/maximum-repeating-substring/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_repeating(sequence: String, word: String) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1668() {
    }
}
