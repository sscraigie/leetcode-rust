/**
 * [1417] Reformat The String
 *
 * You are given an alphanumeric string s. (Alphanumeric string is a string consisting of lowercase English letters and digits).
 * You have to find a permutation of the string where no letter is followed by another letter and no digit is followed by another digit. That is, no two adjacent characters have the same type.
 * Return the reformatted string or return an empty string if it is impossible to reformat the string.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: s = "a0b1c2"
 * Output: "0a1b2c"
 * Explanation: No two adjacent characters have the same type in "0a1b2c". "a0b1c2", "0a1b2c", "0c2a1b" are also valid permutations.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: s = "leetcode"
 * Output: ""
 * Explanation: "leetcode" has only characters so we cannot separate them by digits.
 * 
 * <strong class="example">Example 3:
 * 
 * Input: s = "1229857369"
 * Output: ""
 * Explanation: "1229857369" has only digits so we cannot separate them by characters.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= s.length <= 500
 * 	s consists of only lowercase English letters and/or digits.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/reformat-the-string/
// discuss: https://leetcode.com/problems/reformat-the-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn reformat(s: String) -> String {
        String::new()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1417() {
    }
}
