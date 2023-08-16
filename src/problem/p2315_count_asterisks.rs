/**
 * [2315] Count Asterisks
 *
 * You are given a string s, where every two consecutive vertical bars '|' are grouped into a pair. In other words, the 1^st and 2^nd '|' make a pair, the 3^rd and 4^th '|' make a pair, and so forth.
 * Return the number of '*' in s, excluding the '*' between each pair of '|'.
 * Note that each '|' will belong to exactly one pair.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: s = "l|*e*et|c**o|*de|"
 * Output: 2
 * Explanation: The considered characters are underlined: "<u>l</u>|*e*et|<u>c**o</u>|*de|".
 * The characters between the first and second '|' are excluded from the answer.
 * Also, the characters between the third and fourth '|' are excluded from the answer.
 * There are 2 asterisks considered. Therefore, we return 2.
 * <strong class="example">Example 2:
 * 
 * Input: s = "iamprogrammer"
 * Output: 0
 * Explanation: In this example, there are no asterisks in s. Therefore, we return 0.
 * 
 * <strong class="example">Example 3:
 * 
 * Input: s = "yo|uar|e**|b|e***au|tifu|l"
 * Output: 5
 * Explanation: The considered characters are underlined: "<u>yo</u>|uar|<u>e**</u>|b|<u>e***au</u>|tifu|<u>l</u>". There are 5 asterisks considered. Therefore, we return 5.
 *  
 * Constraints:
 * 
 * 	1 <= s.length <= 1000
 * 	s consists of lowercase English letters, vertical bars '|', and asterisks '*'.
 * 	s contains an even number of vertical bars '|'.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-asterisks/
// discuss: https://leetcode.com/problems/count-asterisks/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_asterisks(s: String) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2315() {
    }
}
