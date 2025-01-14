/**
 * [2684] Maximum Number of Moves in a Grid
 *
 * You are given a 0-indexed m x n matrix grid consisting of positive integers.
 * You can start at any cell in the first column of the matrix, and traverse the grid in the following way:
 * 
 * 	From a cell (row, col), you can move to any of the cells: (row - 1, col + 1), (row, col + 1) and (row + 1, col + 1) such that the value of the cell you move to, should be strictly bigger than the value of the current cell.
 * 
 * Return the maximum number of moves that you can perform.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2023/04/11/yetgriddrawio-10.png" style="width: 201px; height: 201px;" />
 * Input: grid = [[2,4,3,5],[5,4,9,3],[3,4,2,11],[10,9,13,15]]
 * Output: 3
 * Explanation: We can start at the cell (0, 0) and make the following moves:
 * - (0, 0) -> (0, 1).
 * - (0, 1) -> (1, 2).
 * - (1, 2) -> (2, 3).
 * It can be shown that it is the maximum number of moves that can be made.
 * <strong class="example">Example 2:
 * 
 * <img alt="" src="https://assets.leetcode.com/uploads/2023/04/12/yetgrid4drawio.png" />
 * Input: grid = [[3,2,4],[2,1,9],[1,1,7]]
 * Output: 0
 * Explanation: Starting from any cell in the first column we cannot perform any moves.
 * 
 *  
 * Constraints:
 * 
 * 	m == grid.length
 * 	n == grid[i].length
 * 	2 <= m, n <= 1000
 * 	4 <= m * n <= 10^5
 * 	1 <= grid[i][j] <= 10^6
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-number-of-moves-in-a-grid/
// discuss: https://leetcode.com/problems/maximum-number-of-moves-in-a-grid/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2684() {
    }
}
