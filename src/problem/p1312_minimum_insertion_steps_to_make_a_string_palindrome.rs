/**
 * [1312] Minimum Insertion Steps to Make a String Palindrome
 *
 * Given a string s. In one step you can insert any character at any index of the string.
 * Return the minimum number of steps to make s palindrome.
 * A Palindrome String is one that reads the same backward as well as forward.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: s = "zzazz"
 * Output: 0
 * Explanation: The string "zzazz" is already palindrome we do not need any insertions.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: s = "mbadm"
 * Output: 2
 * Explanation: String can be "mbdadbm" or "mdbabdm".
 * 
 * <strong class="example">Example 3:
 * 
 * Input: s = "leetcode"
 * Output: 5
 * Explanation: Inserting 5 characters the string becomes "leetcodocteel".
 * 
 *  
 * Constraints:
 * 
 * 	1 <= s.length <= 500
 * 	s consists of lowercase English letters.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-insertion-steps-to-make-a-string-palindrome/
// discuss: https://leetcode.com/problems/minimum-insertion-steps-to-make-a-string-palindrome/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1312() {
    }
}
