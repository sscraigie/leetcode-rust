/**
 * [2185] Counting Words With a Given Prefix
 *
 * You are given an array of strings words and a string pref.
 * Return the number of strings in words that contain pref as a prefix.
 * A prefix of a string s is any leading contiguous substring of s.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: words = ["pay","<u>at</u>tention","practice","<u>at</u>tend"], pref = "at"
 * Output: 2
 * Explanation: The 2 strings that contain "at" as a prefix are: "<u>at</u>tention" and "<u>at</u>tend".
 * 
 * <strong class="example">Example 2:
 * 
 * Input: words = ["leetcode","win","loops","success"], pref = "code"
 * Output: 0
 * Explanation: There are no strings that contain "code" as a prefix.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= words.length <= 100
 * 	1 <= words[i].length, pref.length <= 100
 * 	words[i] and pref consist of lowercase English letters.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/counting-words-with-a-given-prefix/
// discuss: https://leetcode.com/problems/counting-words-with-a-given-prefix/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2185() {
    }
}
