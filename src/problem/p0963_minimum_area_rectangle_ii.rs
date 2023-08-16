/**
 * [963] Minimum Area Rectangle II
 *
 * You are given an array of points in the X-Y plane points where points[i] = [xi, yi].
 * Return the minimum area of any rectangle formed from these points, with sides not necessarily parallel to the X and Y axes. If there is not any such rectangle, return 0.
 * Answers within 10^-5 of the actual answer will be accepted.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2018/12/21/1a.png" style="width: 398px; height: 400px;" />
 * Input: points = [[1,2],[2,1],[1,0],[0,1]]
 * Output: 2.00000
 * Explanation: The minimum area rectangle occurs at [1,2],[2,1],[1,0],[0,1], with an area of 2.
 * 
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2018/12/22/2.png" style="width: 400px; height: 251px;" />
 * Input: points = [[0,1],[2,1],[1,1],[1,0],[2,0]]
 * Output: 1.00000
 * Explanation: The minimum area rectangle occurs at [1,0],[1,1],[2,1],[2,0], with an area of 1.
 * 
 * <strong class="example">Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2018/12/22/3.png" style="width: 383px; height: 400px;" />
 * Input: points = [[0,3],[1,2],[3,1],[1,3],[2,1]]
 * Output: 0
 * Explanation: There is no possible rectangle to form from these points.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= points.length <= 50
 * 	points[i].length == 2
 * 	0 <= xi, yi <= 4 * 10^4
 * 	All the given points are unique.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-area-rectangle-ii/
// discuss: https://leetcode.com/problems/minimum-area-rectangle-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_area_free_rect(points: Vec<Vec<i32>>) -> f64 {
        0f64
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_963() {
    }
}
