/**
 * [483] Smallest Good Base
 *
 * Given an integer n represented as a string, return the smallest good base of n.
 * We call k >= 2 a good base of n, if all digits of n base k are 1's.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: n = "13"
 * Output: "3"
 * Explanation: 13 base 3 is 111.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: n = "4681"
 * Output: "8"
 * Explanation: 4681 base 8 is 11111.
 * 
 * <strong class="example">Example 3:
 * 
 * Input: n = "1000000000000000000"
 * Output: "999999999999999999"
 * Explanation: 1000000000000000000 base 999999999999999999 is 11.
 * 
 *  
 * Constraints:
 * 
 * 	n is an integer in the range [3, 10^18].
 * 	n does not contain any leading zeros.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/smallest-good-base/
// discuss: https://leetcode.com/problems/smallest-good-base/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn smallest_good_base(n: String) -> String {
        String::new()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_483() {
    }
}
