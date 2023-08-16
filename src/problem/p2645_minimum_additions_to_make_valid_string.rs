/**
 * [2645] Minimum Additions to Make Valid String
 *
 * Given a string word to which you can insert letters "a", "b" or "c" anywhere and any number of times, return the minimum number of letters that must be inserted so that word becomes valid.
 * A string is called valid if it can be formed by concatenating the string "abc" several times.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: word = "b"
 * Output: 2
 * Explanation: Insert the letter "a" right before "b", and the letter "c" right next to "a" to obtain the valid string "abc".
 * 
 * <strong class="example">Example 2:
 * 
 * Input: word = "aaa"
 * Output: 6
 * Explanation: Insert letters "b" and "c" next to each "a" to obtain the valid string "abcabcabc".
 * 
 * <strong class="example">Example 3:
 * 
 * Input: word = "abc"
 * Output: 0
 * Explanation: word is already valid. No modifications are needed. 
 * 
 *  
 * Constraints:
 * 
 * 	1 <= word.length <= 50
 * 	word consists of letters "a", "b" and "c" only. 
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-additions-to-make-valid-string/
// discuss: https://leetcode.com/problems/minimum-additions-to-make-valid-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn add_minimum(word: String) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2645() {
    }
}
