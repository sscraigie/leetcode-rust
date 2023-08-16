/**
 * [2249] Count Lattice Points Inside a Circle
 *
 * Given a 2D integer array circles where circles[i] = [xi, yi, ri] represents the center (xi, yi) and radius ri of the i^th circle drawn on a grid, return the number of lattice points that are present inside at least one circle.
 * Note:
 * 
 * 	A lattice point is a point with integer coordinates.
 * 	Points that lie on the circumference of a circle are also considered to be inside it.
 * 
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/03/02/exa-11.png" style="width: 300px; height: 300px;" />
 * Input: circles = [[2,2,1]]
 * Output: 5
 * Explanation:
 * The figure above shows the given circle.
 * The lattice points present inside the circle are (1, 2), (2, 1), (2, 2), (2, 3), and (3, 2) and are shown in green.
 * Other points such as (1, 1) and (1, 3), which are shown in red, are not considered inside the circle.
 * Hence, the number of lattice points present inside at least one circle is 5.
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/03/02/exa-22.png" style="width: 300px; height: 300px;" />
 * Input: circles = [[2,2,2],[3,4,1]]
 * Output: 16
 * Explanation:
 * The figure above shows the given circles.
 * There are exactly 16 lattice points which are present inside at least one circle. 
 * Some of them are (0, 2), (2, 0), (2, 4), (3, 2), and (4, 4).
 * 
 *  
 * Constraints:
 * 
 * 	1 <= circles.length <= 200
 * 	circles[i].length == 3
 * 	1 <= xi, yi <= 100
 * 	1 <= ri <= min(xi, yi)
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-lattice-points-inside-a-circle/
// discuss: https://leetcode.com/problems/count-lattice-points-inside-a-circle/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_lattice_points(circles: Vec<Vec<i32>>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2249() {
    }
}
