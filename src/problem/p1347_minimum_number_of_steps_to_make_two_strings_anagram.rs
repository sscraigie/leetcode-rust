/**
 * [1347] Minimum Number of Steps to Make Two Strings Anagram
 *
 * You are given two strings of the same length s and t. In one step you can choose any character of t and replace it with another character.
 * Return the minimum number of steps to make t an anagram of s.
 * An Anagram of a string is a string that contains the same characters with a different (or the same) ordering.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: s = "bab", t = "aba"
 * Output: 1
 * Explanation: Replace the first 'a' in t with b, t = "bba" which is anagram of s.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: s = "leetcode", t = "practice"
 * Output: 5
 * Explanation: Replace 'p', 'r', 'a', 'i' and 'c' from t with proper characters to make t anagram of s.
 * 
 * <strong class="example">Example 3:
 * 
 * Input: s = "anagram", t = "mangaar"
 * Output: 0
 * Explanation: "anagram" and "mangaar" are anagrams. 
 * 
 *  
 * Constraints:
 * 
 * 	1 <= s.length <= 5 * 10^4
 * 	s.length == t.length
 * 	s and t consist of lowercase English letters only.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-number-of-steps-to-make-two-strings-anagram/
// discuss: https://leetcode.com/problems/minimum-number-of-steps-to-make-two-strings-anagram/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1347() {
    }
}
