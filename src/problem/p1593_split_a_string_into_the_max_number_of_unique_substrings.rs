/**
 * [1593] Split a String Into the Max Number of Unique Substrings
 *
 * Given a string s<var>,</var> return the maximum number of unique substrings that the given string can be split into.
 * You can split string s into any list of non-empty substrings, where the concatenation of the substrings forms the original string. However, you must split the substrings such that all of them are unique.
 * A substring is a contiguous sequence of characters within a string.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: s = "ababccc"
 * Output: 5
 * Explanation: One way to split maximally is ['a', 'b', 'ab', 'c', 'cc']. Splitting like ['a', 'b', 'a', 'b', 'c', 'cc'] is not valid as you have 'a' and 'b' multiple times.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: s = "aba"
 * Output: 2
 * Explanation: One way to split maximally is ['a', 'ba'].
 * 
 * <strong class="example">Example 3:
 * 
 * Input: s = "aa"
 * Output: 1
 * Explanation: It is impossible to split the string any further.
 * 
 *  
 * Constraints:
 * 
 * 	
 * 	1 <= s.length <= 16
 * 	
 * 	
 * 	s contains only lower case English letters.
 * 	
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/split-a-string-into-the-max-number-of-unique-substrings/
// discuss: https://leetcode.com/problems/split-a-string-into-the-max-number-of-unique-substrings/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_unique_split(s: String) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1593() {
    }
}
