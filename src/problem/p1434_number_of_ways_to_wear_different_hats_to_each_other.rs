/**
 * [1434] Number of Ways to Wear Different Hats to Each Other
 *
 * There are n people and 40 types of hats labeled from 1 to 40.
 * Given a 2D integer array hats, where hats[i] is a list of all hats preferred by the i^th person.
 * Return the number of ways that the n people wear different hats to each other.
 * Since the answer may be too large, return it modulo 10^9 + 7.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: hats = [[3,4],[4,5],[5]]
 * Output: 1
 * Explanation: There is only one way to choose hats given the conditions. 
 * First person choose hat 3, Second person choose hat 4 and last one hat 5.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: hats = [[3,5,1],[3,5]]
 * Output: 4
 * Explanation: There are 4 ways to choose hats:
 * (3,5), (5,3), (1,3) and (1,5)
 * 
 * <strong class="example">Example 3:
 * 
 * Input: hats = [[1,2,3,4],[1,2,3,4],[1,2,3,4],[1,2,3,4]]
 * Output: 24
 * Explanation: Each person can choose hats labeled from 1 to 4.
 * Number of Permutations of (1,2,3,4) = 24.
 * 
 *  
 * Constraints:
 * 
 * 	n == hats.length
 * 	1 <= n <= 10
 * 	1 <= hats[i].length <= 40
 * 	1 <= hats[i][j] <= 40
 * 	hats[i] contains a list of unique integers.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-ways-to-wear-different-hats-to-each-other/
// discuss: https://leetcode.com/problems/number-of-ways-to-wear-different-hats-to-each-other/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn number_ways(hats: Vec<Vec<i32>>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1434() {
    }
}
