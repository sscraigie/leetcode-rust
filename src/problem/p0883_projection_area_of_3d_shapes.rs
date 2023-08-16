/**
 * [883] Projection Area of 3D Shapes
 *
 * You are given an n x n grid where we place some 1 x 1 x 1 cubes that are axis-aligned with the x, y, and z axes.
 * Each value v = grid[i][j] represents a tower of v cubes placed on top of the cell (i, j).
 * We view the projection of these cubes onto the xy, yz, and zx planes.
 * A projection is like a shadow, that maps our 3-dimensional figure to a 2-dimensional plane. We are viewing the "shadow" when looking at the cubes from the top, the front, and the side.
 * Return the total area of all three projections.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://s3-lc-upload.s3.amazonaws.com/uploads/2018/08/02/shadow.png" style="width: 800px; height: 214px;" />
 * Input: grid = [[1,2],[3,4]]
 * Output: 17
 * Explanation: Here are the three projections ("shadows") of the shape made with each axis-aligned plane.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: grid = [[2]]
 * Output: 5
 * 
 * <strong class="example">Example 3:
 * 
 * Input: grid = [[1,0],[0,2]]
 * Output: 8
 * 
 *  
 * Constraints:
 * 
 * 	n == grid.length == grid[i].length
 * 	1 <= n <= 50
 * 	0 <= grid[i][j] <= 50
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/projection-area-of-3d-shapes/
// discuss: https://leetcode.com/problems/projection-area-of-3d-shapes/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn projection_area(grid: Vec<Vec<i32>>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_883() {
    }
}
