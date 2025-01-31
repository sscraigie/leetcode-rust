/**
 * [1156] Swap For Longest Repeated Character Substring
 *
 * You are given a string text. You can swap two of the characters in the text.
 * Return the length of the longest substring with repeated characters.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: text = "ababa"
 * Output: 3
 * Explanation: We can swap the first 'b' with the last 'a', or the last 'b' with the first 'a'. Then, the longest repeated character substring is "aaa" with length 3.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: text = "aaabaaa"
 * Output: 6
 * Explanation: Swap 'b' with the last 'a' (or the first 'a'), and we get longest repeated character substring "aaaaaa" with length 6.
 * 
 * <strong class="example">Example 3:
 * 
 * Input: text = "aaaaa"
 * Output: 5
 * Explanation: No need to swap, longest repeated character substring is "aaaaa" with length is 5.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= text.length <= 2 * 10^4
 * 	text consist of lowercase English characters only.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/swap-for-longest-repeated-character-substring/
// discuss: https://leetcode.com/problems/swap-for-longest-repeated-character-substring/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_rep_opt1(text: String) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1156() {
    }
}
