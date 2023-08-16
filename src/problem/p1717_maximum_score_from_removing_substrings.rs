/**
 * [1717] Maximum Score From Removing Substrings
 *
 * You are given a string s and two integers x and y. You can perform two types of operations any number of times.
 * 
 * 	Remove substring "ab" and gain x points.
 * 	
 * 		For example, when removing "ab" from "c<u>ab</u>xbae" it becomes "cxbae".
 * 	
 * 	
 * 	Remove substring "ba" and gain y points.
 * 	
 * 		For example, when removing "ba" from "cabx<u>ba</u>e" it becomes "cabxe".
 * 	
 * 	
 * 
 * Return the maximum points you can gain after applying the above operations on s.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: s = "cdbcbbaaabab", x = 4, y = 5
 * Output: 19
 * Explanation:
 * - Remove the "ba" underlined in "cdbcbbaaa<u>ba</u>b". Now, s = "cdbcbbaaab" and 5 points are added to the score.
 * - Remove the "ab" underlined in "cdbcbbaa<u>ab</u>". Now, s = "cdbcbbaa" and 4 points are added to the score.
 * - Remove the "ba" underlined in "cdbcb<u>ba</u>a". Now, s = "cdbcba" and 5 points are added to the score.
 * - Remove the "ba" underlined in "cdbc<u>ba</u>". Now, s = "cdbc" and 5 points are added to the score.
 * Total score = 5 + 4 + 5 + 5 = 19.
 * <strong class="example">Example 2:
 * 
 * Input: s = "aabbaaxybbaabb", x = 5, y = 4
 * Output: 20
 * 
 *  
 * Constraints:
 * 
 * 	1 <= s.length <= 10^5
 * 	1 <= x, y <= 10^4
 * 	s consists of lowercase English letters.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-score-from-removing-substrings/
// discuss: https://leetcode.com/problems/maximum-score-from-removing-substrings/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1717() {
    }
}
