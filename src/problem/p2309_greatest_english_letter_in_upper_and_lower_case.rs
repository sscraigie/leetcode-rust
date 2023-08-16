/**
 * [2309] Greatest English Letter in Upper and Lower Case
 *
 * Given a string of English letters s, return the greatest English letter which occurs as both a lowercase and uppercase letter in s. The returned letter should be in uppercase. If no such letter exists, return an empty string.
 * An English letter b is greater than another letter a if b appears after a in the English alphabet.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: s = "l<u>Ee</u>TcOd<u>E</u>"
 * Output: "E"
 * Explanation:
 * The letter 'E' is the only letter to appear in both lower and upper case.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: s = "a<u>rR</u>AzFif"
 * Output: "R"
 * Explanation:
 * The letter 'R' is the greatest letter to appear in both lower and upper case.
 * Note that 'A' and 'F' also appear in both lower and upper case, but 'R' is greater than 'F' or 'A'.
 * 
 * <strong class="example">Example 3:
 * 
 * Input: s = "AbCdEfGhIjK"
 * Output: ""
 * Explanation:
 * There is no letter that appears in both lower and upper case.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= s.length <= 1000
 * 	s consists of lowercase and uppercase English letters.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/greatest-english-letter-in-upper-and-lower-case/
// discuss: https://leetcode.com/problems/greatest-english-letter-in-upper-and-lower-case/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn greatest_letter(s: String) -> String {
        String::new()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2309() {
    }
}
