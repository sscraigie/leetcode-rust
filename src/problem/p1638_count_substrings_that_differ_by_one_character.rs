/**
 * [1638] Count Substrings That Differ by One Character
 *
 * Given two strings s and t, find the number of ways you can choose a non-empty substring of s and replace a single character by a different character such that the resulting substring is a substring of t. In other words, find the number of substrings in s that differ from some substring in t by exactly one character.
 * For example, the underlined substrings in "<u>compute</u>r" and "<u>computa</u>tion" only differ by the 'e'/'a', so this is a valid way.
 * Return the number of substrings that satisfy the condition above.
 * A substring is a contiguous sequence of characters within a string.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: s = "aba", t = "baba"
 * Output: 6
 * Explanation: The following are the pairs of substrings from s and t that differ by exactly 1 character:
 * ("<u>a</u>ba", "<u>b</u>aba")
 * ("<u>a</u>ba", "ba<u>b</u>a")
 * ("ab<u>a</u>", "<u>b</u>aba")
 * ("ab<u>a</u>", "ba<u>b</u>a")
 * ("a<u>b</u>a", "b<u>a</u>ba")
 * ("a<u>b</u>a", "bab<u>a</u>")
 * The underlined portions are the substrings that are chosen from s and t.
 * ​​<strong class="example">Example 2:
 * 
 * Input: s = "ab", t = "bb"
 * Output: 3
 * Explanation: The following are the pairs of substrings from s and t that differ by 1 character:
 * ("<u>a</u>b", "<u>b</u>b")
 * ("<u>a</u>b", "b<u>b</u>")
 * ("<u>ab</u>", "<u>bb</u>")
 * ​​​​The underlined portions are the substrings that are chosen from s and t.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= s.length, t.length <= 100
 * 	s and t consist of lowercase English letters only.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-substrings-that-differ-by-one-character/
// discuss: https://leetcode.com/problems/count-substrings-that-differ-by-one-character/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_substrings(s: String, t: String) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1638() {
    }
}
