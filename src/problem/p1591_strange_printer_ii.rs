/**
 * [1591] Strange Printer II
 *
 * There is a strange printer with the following two special requirements:
 * 
 * 	On each turn, the printer will print a solid rectangular pattern of a single color on the grid. This will cover up the existing colors in the rectangle.
 * 	Once the printer has used a color for the above operation, the same color cannot be used again.
 * 
 * You are given a m x n matrix targetGrid, where targetGrid[row][col] is the color in the position (row, col) of the grid.
 * Return true if it is possible to print the matrix targetGrid, otherwise, return false.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/12/23/print1.jpg" style="width: 600px; height: 175px;" />
 * Input: targetGrid = [[1,1,1,1],[1,2,2,1],[1,2,2,1],[1,1,1,1]]
 * Output: true
 * 
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/12/23/print2.jpg" style="width: 600px; height: 367px;" />
 * Input: targetGrid = [[1,1,1,1],[1,1,3,3],[1,1,3,4],[5,5,1,4]]
 * Output: true
 * 
 * <strong class="example">Example 3:
 * 
 * Input: targetGrid = [[1,2,1],[2,1,2],[1,2,1]]
 * Output: false
 * Explanation: It is impossible to form targetGrid because it is not allowed to print the same color in different turns.
 * 
 *  
 * Constraints:
 * 
 * 	m == targetGrid.length
 * 	n == targetGrid[i].length
 * 	1 <= m, n <= 60
 * 	1 <= targetGrid[row][col] <= 60
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/strange-printer-ii/
// discuss: https://leetcode.com/problems/strange-printer-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_printable(target_grid: Vec<Vec<i32>>) -> bool {
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1591() {
    }
}
