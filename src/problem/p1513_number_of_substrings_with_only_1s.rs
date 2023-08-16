/**
 * [1513] Number of Substrings With Only 1s
 *
 * Given a binary string s, return the number of substrings with all characters 1's. Since the answer may be too large, return it modulo 10^9 + 7.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: s = "0110111"
 * Output: 9
 * Explanation: There are 9 substring in total with only 1's characters.
 * "1" -> 5 times.
 * "11" -> 3 times.
 * "111" -> 1 time.
 * <strong class="example">Example 2:
 * 
 * Input: s = "101"
 * Output: 2
 * Explanation: Substring "1" is shown 2 times in s.
 * 
 * <strong class="example">Example 3:
 * 
 * Input: s = "111111"
 * Output: 21
 * Explanation: Each substring contains only 1's characters.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= s.length <= 10^5
 * 	s[i] is either '0' or '1'.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-substrings-with-only-1s/
// discuss: https://leetcode.com/problems/number-of-substrings-with-only-1s/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn num_sub(s: String) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1513() {
    }
}
