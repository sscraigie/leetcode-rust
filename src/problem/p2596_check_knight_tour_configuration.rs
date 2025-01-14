/**
 * [2596] Check Knight Tour Configuration
 *
 * There is a knight on an n x n chessboard. In a valid configuration, the knight starts at the top-left cell of the board and visits every cell on the board exactly once.
 * You are given an n x n integer matrix grid consisting of distinct integers from the range [0, n * n - 1] where grid[row][col] indicates that the cell (row, col) is the grid[row][col]^th cell that the knight visited. The moves are 0-indexed.
 * Return true if grid represents a valid configuration of the knight's movements or false otherwise.
 * Note that a valid knight move consists of moving two squares vertically and one square horizontally, or two squares horizontally and one square vertically. The figure below illustrates all the possible eight moves of a knight from some cell.
 * <img alt="" src="https://assets.leetcode.com/uploads/2018/10/12/knight.png" style="width: 300px; height: 300px;" />
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/12/28/yetgriddrawio-5.png" style="width: 251px; height: 251px;" />
 * Input: grid = [[0,11,16,5,20],[17,4,19,10,15],[12,1,8,21,6],[3,18,23,14,9],[24,13,2,7,22]]
 * Output: true
 * Explanation: The above diagram represents the grid. It can be shown that it is a valid configuration.
 * 
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/12/28/yetgriddrawio-6.png" style="width: 151px; height: 151px;" />
 * Input: grid = [[0,3,6],[5,8,1],[2,7,4]]
 * Output: false
 * Explanation: The above diagram represents the grid. The 8^th move of the knight is not valid considering its position after the 7^th move.
 * 
 *  
 * Constraints:
 * 
 * 	n == grid.length == grid[i].length
 * 	3 <= n <= 7
 * 	0 <= grid[row][col] < n * n
 * 	All integers in grid are unique.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/check-knight-tour-configuration/
// discuss: https://leetcode.com/problems/check-knight-tour-configuration/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn check_valid_grid(grid: Vec<Vec<i32>>) -> bool {
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2596() {
    }
}
