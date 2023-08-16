/**
 * [564] Find the Closest Palindrome
 *
 * Given a string n representing an integer, return the closest integer (not including itself), which is a palindrome. If there is a tie, return the smaller one.
 * The closest is defined as the absolute difference minimized between two integers.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: n = "123"
 * Output: "121"
 * 
 * <strong class="example">Example 2:
 * 
 * Input: n = "1"
 * Output: "0"
 * Explanation: 0 and 2 are the closest palindromes but we return the smallest which is 0.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= n.length <= 18
 * 	n consists of only digits.
 * 	n does not have leading zeros.
 * 	n is representing an integer in the range [1, 10^18 - 1].
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-the-closest-palindrome/
// discuss: https://leetcode.com/problems/find-the-closest-palindrome/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn nearest_palindromic(n: String) -> String {
        String::new()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_564() {
    }
}
