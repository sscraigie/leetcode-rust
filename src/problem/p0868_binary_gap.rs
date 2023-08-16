/**
 * [868] Binary Gap
 *
 * Given a positive integer n, find and return the longest distance between any two adjacent 1's in the binary representation of n. If there are no two adjacent 1's, return 0.
 * Two 1's are adjacent if there are only 0's separating them (possibly no 0's). The distance between two 1's is the absolute difference between their bit positions. For example, the two 1's in "1001" have a distance of 3.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: n = 22
 * Output: 2
 * Explanation: 22 in binary is "10110".
 * The first adjacent pair of 1's is "<u>1</u>0<u>1</u>10" with a distance of 2.
 * The second adjacent pair of 1's is "10<u>11</u>0" with a distance of 1.
 * The answer is the largest of these two distances, which is 2.
 * Note that "<u>1</u>01<u>1</u>0" is not a valid pair since there is a 1 separating the two 1's underlined.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: n = 8
 * Output: 0
 * Explanation: 8 in binary is "1000".
 * There are not any adjacent pairs of 1's in the binary representation of 8, so we return 0.
 * 
 * <strong class="example">Example 3:
 * 
 * Input: n = 5
 * Output: 2
 * Explanation: 5 in binary is "101".
 * 
 *  
 * Constraints:
 * 
 * 	1 <= n <= 10^9
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/binary-gap/
// discuss: https://leetcode.com/problems/binary-gap/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn binary_gap(n: i32) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_868() {
    }
}
