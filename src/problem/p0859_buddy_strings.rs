/**
 * [859] Buddy Strings
 *
 * Given two strings s and goal, return true if you can swap two letters in s so the result is equal to goal, otherwise, return false.
 * Swapping letters is defined as taking two indices i and j (0-indexed) such that i != j and swapping the characters at s[i] and s[j].
 * 
 * 	For example, swapping at indices 0 and 2 in "abcd" results in "cbad".
 * 
 *  
 * <strong class="example">Example 1:
 * 
 * Input: s = "ab", goal = "ba"
 * Output: true
 * Explanation: You can swap s[0] = 'a' and s[1] = 'b' to get "ba", which is equal to goal.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: s = "ab", goal = "ab"
 * Output: false
 * Explanation: The only letters you can swap are s[0] = 'a' and s[1] = 'b', which results in "ba" != goal.
 * 
 * <strong class="example">Example 3:
 * 
 * Input: s = "aa", goal = "aa"
 * Output: true
 * Explanation: You can swap s[0] = 'a' and s[1] = 'a' to get "aa", which is equal to goal.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= s.length, goal.length <= 2 * 10^4
 * 	s and goal consist of lowercase letters.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/buddy-strings/
// discuss: https://leetcode.com/problems/buddy-strings/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn buddy_strings(s: String, goal: String) -> bool {
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_859() {
    }
}
