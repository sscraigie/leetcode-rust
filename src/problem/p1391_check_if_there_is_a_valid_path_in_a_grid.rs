/**
 * [1391] Check if There is a Valid Path in a Grid
 *
 * You are given an m x n grid. Each cell of grid represents a street. The street of grid[i][j] can be:
 * 
 * 	1 which means a street connecting the left cell and the right cell.
 * 	2 which means a street connecting the upper cell and the lower cell.
 * 	3 which means a street connecting the left cell and the lower cell.
 * 	4 which means a street connecting the right cell and the lower cell.
 * 	5 which means a street connecting the left cell and the upper cell.
 * 	6 which means a street connecting the right cell and the upper cell.
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/03/05/main.png" style="width: 450px; height: 708px;" />
 * You will initially start at the street of the upper-left cell (0, 0). A valid path in the grid is a path that starts from the upper left cell (0, 0) and ends at the bottom-right cell (m - 1, n - 1). The path should only follow the streets.
 * Notice that you are not allowed to change any street.
 * Return true if there is a valid path in the grid or false otherwise.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/03/05/e1.png" style="width: 455px; height: 311px;" />
 * Input: grid = [[2,4,3],[6,5,2]]
 * Output: true
 * Explanation: As shown you can start at cell (0, 0) and visit all the cells of the grid to reach (m - 1, n - 1).
 * 
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/03/05/e2.png" style="width: 455px; height: 293px;" />
 * Input: grid = [[1,2,1],[1,2,1]]
 * Output: false
 * Explanation: As shown you the street at cell (0, 0) is not connected with any street of any other cell and you will get stuck at cell (0, 0)
 * 
 * <strong class="example">Example 3:
 * 
 * Input: grid = [[1,1,2]]
 * Output: false
 * Explanation: You will get stuck at cell (0, 1) and you cannot reach cell (0, 2).
 * 
 *  
 * Constraints:
 * 
 * 	m == grid.length
 * 	n == grid[i].length
 * 	1 <= m, n <= 300
 * 	1 <= grid[i][j] <= 6
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/check-if-there-is-a-valid-path-in-a-grid/
// discuss: https://leetcode.com/problems/check-if-there-is-a-valid-path-in-a-grid/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn has_valid_path(grid: Vec<Vec<i32>>) -> bool {
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1391() {
    }
}
