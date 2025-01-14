/**
 * [1078] Occurrences After Bigram
 *
 * Given two strings first and second, consider occurrences in some text of the form "first second third", where second comes immediately after first, and third comes immediately after second.
 * Return an array of all the words third for each occurrence of "first second third".
 *  
 * <strong class="example">Example 1:
 * Input: text = "alice is a good girl she is a good student", first = "a", second = "good"
 * Output: ["girl","student"]
 * <strong class="example">Example 2:
 * Input: text = "we will we will rock you", first = "we", second = "will"
 * Output: ["we","rock"]
 *  
 * Constraints:
 * 
 * 	1 <= text.length <= 1000
 * 	text consists of lowercase English letters and spaces.
 * 	All the words in text a separated by a single space.
 * 	1 <= first.length, second.length <= 10
 * 	first and second consist of lowercase English letters.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/occurrences-after-bigram/
// discuss: https://leetcode.com/problems/occurrences-after-bigram/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_ocurrences(text: String, first: String, second: String) -> Vec<String> {
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1078() {
    }
}
