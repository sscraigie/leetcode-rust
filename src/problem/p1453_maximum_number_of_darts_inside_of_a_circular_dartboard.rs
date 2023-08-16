/**
 * [1453] Maximum Number of Darts Inside of a Circular Dartboard
 *
 * Alice is throwing n darts on a very large wall. You are given an array darts where darts[i] = [xi, yi] is the position of the i^th dart that Alice threw on the wall.
 * Bob knows the positions of the n darts on the wall. He wants to place a dartboard of radius r on the wall so that the maximum number of darts that Alice throws lie on the dartboard.
 * Given the integer r, return the maximum number of darts that can lie on the dartboard.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/04/29/sample_1_1806.png" style="width: 248px; height: 211px;" />
 * Input: darts = [[-2,0],[2,0],[0,2],[0,-2]], r = 2
 * Output: 4
 * Explanation: Circle dartboard with center in (0,0) and radius = 2 contain all points.
 * 
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/04/29/sample_2_1806.png" style="width: 306px; height: 244px;" />
 * Input: darts = [[-3,0],[3,0],[2,6],[5,4],[0,9],[7,8]], r = 5
 * Output: 5
 * Explanation: Circle dartboard with center in (0,4) and radius = 5 contain all points except the point (7,8).
 * 
 *  
 * Constraints:
 * 
 * 	1 <= darts.length <= 100
 * 	darts[i].length == 2
 * 	-10^4 <= xi, yi <= 10^4
 * 	All the darts are unique
 * 	1 <= r <= 5000
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-number-of-darts-inside-of-a-circular-dartboard/
// discuss: https://leetcode.com/problems/maximum-number-of-darts-inside-of-a-circular-dartboard/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn num_points(darts: Vec<Vec<i32>>, r: i32) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1453() {
    }
}
