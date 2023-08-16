/**
 * [880] Decoded String at Index
 *
 * You are given an encoded string s. To decode the string to a tape, the encoded string is read one character at a time and the following steps are taken:
 * 
 * 	If the character read is a letter, that letter is written onto the tape.
 * 	If the character read is a digit d, the entire current tape is repeatedly written d - 1 more times in total.
 * 
 * Given an integer k, return the k^th letter (1-indexed) in the decoded string.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: s = "leet2code3", k = 10
 * Output: "o"
 * Explanation: The decoded string is "leetleetcodeleetleetcodeleetleetcode".
 * The 10^th letter in the string is "o".
 * 
 * <strong class="example">Example 2:
 * 
 * Input: s = "ha22", k = 5
 * Output: "h"
 * Explanation: The decoded string is "hahahaha".
 * The 5^th letter is "h".
 * 
 * <strong class="example">Example 3:
 * 
 * Input: s = "a2345678999999999999999", k = 1
 * Output: "a"
 * Explanation: The decoded string is "a" repeated 8301530446056247680 times.
 * The 1^st letter is "a".
 * 
 *  
 * Constraints:
 * 
 * 	2 <= s.length <= 100
 * 	s consists of lowercase English letters and digits 2 through 9.
 * 	s starts with a letter.
 * 	1 <= k <= 10^9
 * 	It is guaranteed that k is less than or equal to the length of the decoded string.
 * 	The decoded string is guaranteed to have less than 2^63 letters.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/decoded-string-at-index/
// discuss: https://leetcode.com/problems/decoded-string-at-index/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn decode_at_index(s: String, k: i32) -> String {
        String::new()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_880() {
    }
}