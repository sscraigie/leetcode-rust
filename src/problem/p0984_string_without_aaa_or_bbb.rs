/**
 * [984] String Without AAA or BBB
 *
 * Given two integers a and b, return any string s such that:
 * 
 * 	s has length a + b and contains exactly a 'a' letters, and exactly b 'b' letters,
 * 	The substring 'aaa' does not occur in s, and
 * 	The substring 'bbb' does not occur in s.
 * 
 *  
 * <strong class="example">Example 1:
 * 
 * Input: a = 1, b = 2
 * Output: "abb"
 * Explanation: "abb", "bab" and "bba" are all correct answers.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: a = 4, b = 1
 * Output: "aabaa"
 * 
 *  
 * Constraints:
 * 
 * 	0 <= a, b <= 100
 * 	It is guaranteed such an s exists for the given a and b.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/string-without-aaa-or-bbb/
// discuss: https://leetcode.com/problems/string-without-aaa-or-bbb/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn str_without3a3b(a: i32, b: i32) -> String {
        String::new()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_984() {
    }
}
