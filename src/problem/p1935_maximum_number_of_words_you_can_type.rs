/**
 * [1935] Maximum Number of Words You Can Type
 *
 * There is a malfunctioning keyboard where some letter keys do not work. All other keys on the keyboard work properly.
 * Given a string text of words separated by a single space (no leading or trailing spaces) and a string brokenLetters of all distinct letter keys that are broken, return the number of words in text you can fully type using this keyboard.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: text = "hello world", brokenLetters = "ad"
 * Output: 1
 * Explanation: We cannot type "world" because the 'd' key is broken.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: text = "leet code", brokenLetters = "lt"
 * Output: 1
 * Explanation: We cannot type "leet" because the 'l' and 't' keys are broken.
 * 
 * <strong class="example">Example 3:
 * 
 * Input: text = "leet code", brokenLetters = "e"
 * Output: 0
 * Explanation: We cannot type either word because the 'e' key is broken.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= text.length <= 10^4
 * 	0 <= brokenLetters.length <= 26
 * 	text consists of words separated by a single space without any leading or trailing spaces.
 * 	Each word only consists of lowercase English letters.
 * 	brokenLetters consists of distinct lowercase English letters.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-number-of-words-you-can-type/
// discuss: https://leetcode.com/problems/maximum-number-of-words-you-can-type/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1935() {
    }
}
