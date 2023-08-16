/**
 * [1278] Palindrome Partitioning III
 *
 * You are given a string s containing lowercase letters and an integer k. You need to :
 * 
 * 	First, change some characters of s to other lowercase English letters.
 * 	Then divide s into k non-empty disjoint substrings such that each substring is a palindrome.
 * 
 * Return the minimal number of characters that you need to change to divide the string.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: s = "abc", k = 2
 * Output: 1
 * Explanation: You can split the string into "ab" and "c", and change 1 character in "ab" to make it palindrome.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: s = "aabbc", k = 3
 * Output: 0
 * Explanation: You can split the string into "aa", "bb" and "c", all of them are palindrome.
 * <strong class="example">Example 3:
 * 
 * Input: s = "leetcode", k = 8
 * Output: 0
 * 
 *  
 * Constraints:
 * 
 * 	1 <= k <= s.length <= 100.
 * 	s only contains lowercase English letters.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/palindrome-partitioning-iii/
// discuss: https://leetcode.com/problems/palindrome-partitioning-iii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn palindrome_partition(s: String, k: i32) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1278() {
    }
}
