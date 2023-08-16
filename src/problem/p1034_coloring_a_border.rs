/**
 * [1034] Coloring A Border
 *
 * You are given an m x n integer matrix grid, and three integers row, col, and color. Each value in the grid represents the color of the grid square at that location.
 * Two squares are called adjacent if they are next to each other in any of the 4 directions.
 * Two squares belong to the same connected component if they have the same color and they are adjacent.
 * The border of a connected component is all the squares in the connected component that are either adjacent to (at least) a square not in the component, or on the boundary of the grid (the first or last row or column).
 * You should color the border of the connected component that contains the square grid[row][col] with color.
 * Return the final grid.
 *  
 * <strong class="example">Example 1:
 * Input: grid = [[1,1],[1,2]], row = 0, col = 0, color = 3
 * Output: [[3,3],[3,2]]
 * <strong class="example">Example 2:
 * Input: grid = [[1,2,2],[2,3,2]], row = 0, col = 1, color = 3
 * Output: [[1,3,3],[2,3,3]]
 * <strong class="example">Example 3:
 * Input: grid = [[1,1,1],[1,1,1],[1,1,1]], row = 1, col = 1, color = 2
 * Output: [[2,2,2],[2,1,2],[2,2,2]]
 *  
 * Constraints:
 * 
 * 	m == grid.length
 * 	n == grid[i].length
 * 	1 <= m, n <= 50
 * 	1 <= grid[i][j], color <= 1000
 * 	0 <= row < m
 * 	0 <= col < n
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/coloring-a-border/
// discuss: https://leetcode.com/problems/coloring-a-border/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn color_border(grid: Vec<Vec<i32>>, row: i32, col: i32, color: i32) -> Vec<Vec<i32>> {
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1034() {
    }
}
