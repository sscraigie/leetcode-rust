/**
 * [1866] Number of Ways to Rearrange Sticks With K Sticks Visible
 *
 * There are n uniquely-sized sticks whose lengths are integers from 1 to n. You want to arrange the sticks such that exactly k sticks are visible from the left. A stick is visible from the left if there are no longer sticks to the left of it.
 * 
 * 	For example, if the sticks are arranged [<u>1</u>,<u>3</u>,2,<u>5</u>,4], then the sticks with lengths 1, 3, and 5 are visible from the left.
 * 
 * Given n and k, return the number of such arrangements. Since the answer may be large, return it modulo 10^9 + 7.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: n = 3, k = 2
 * Output: 3
 * Explanation: [<u>1</u>,<u>3</u>,2], [<u>2</u>,<u>3</u>,1], and [<u>2</u>,1,<u>3</u>] are the only arrangements such that exactly 2 sticks are visible.
 * The visible sticks are underlined.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: n = 5, k = 5
 * Output: 1
 * Explanation: [<u>1</u>,<u>2</u>,<u>3</u>,<u>4</u>,<u>5</u>] is the only arrangement such that all 5 sticks are visible.
 * The visible sticks are underlined.
 * 
 * <strong class="example">Example 3:
 * 
 * Input: n = 20, k = 11
 * Output: 647427950
 * Explanation: There are 647427950 (mod 10^9 + 7) ways to rearrange the sticks such that exactly 11 sticks are visible.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= n <= 1000
 * 	1 <= k <= n
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-ways-to-rearrange-sticks-with-k-sticks-visible/
// discuss: https://leetcode.com/problems/number-of-ways-to-rearrange-sticks-with-k-sticks-visible/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn rearrange_sticks(n: i32, k: i32) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1866() {
    }
}
