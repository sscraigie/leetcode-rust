/**
 * [1007] Minimum Domino Rotations For Equal Row
 *
 * In a row of dominoes, tops[i] and bottoms[i] represent the top and bottom halves of the i^th domino. (A domino is a tile with two numbers from 1 to 6 - one on each half of the tile.)
 * We may rotate the i^th domino, so that tops[i] and bottoms[i] swap values.
 * Return the minimum number of rotations so that all the values in tops are the same, or all the values in bottoms are the same.
 * If it cannot be done, return -1.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/05/14/domino.png" style="height: 300px; width: 421px;" />
 * Input: tops = [2,1,2,4,2,2], bottoms = [5,2,6,2,3,2]
 * Output: 2
 * Explanation: 
 * The first figure represents the dominoes as given by tops and bottoms: before we do any rotations.
 * If we rotate the second and fourth dominoes, we can make every value in the top row equal to 2, as indicated by the second figure.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: tops = [3,5,1,2,3], bottoms = [3,6,3,3,4]
 * Output: -1
 * Explanation: 
 * In this case, it is not possible to rotate the dominoes to make one row of values equal.
 * 
 *  
 * Constraints:
 * 
 * 	2 <= tops.length <= 2 * 10^4
 * 	bottoms.length == tops.length
 * 	1 <= tops[i], bottoms[i] <= 6
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-domino-rotations-for-equal-row/
// discuss: https://leetcode.com/problems/minimum-domino-rotations-for-equal-row/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1007() {
    }
}
