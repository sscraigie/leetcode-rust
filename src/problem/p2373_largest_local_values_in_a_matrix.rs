/**
 * [2373] Largest Local Values in a Matrix
 *
 * You are given an n x n integer matrix grid.
 * Generate an integer matrix maxLocal of size (n - 2) x (n - 2) such that:
 * 
 * 	maxLocal[i][j] is equal to the largest value of the 3 x 3 matrix in grid centered around row i + 1 and column j + 1.
 * 
 * In other words, we want to find the largest value in every contiguous 3 x 3 matrix in grid.
 * Return the generated matrix.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/06/21/ex1.png" style="width: 371px; height: 210px;" />
 * Input: grid = [[9,9,8,1],[5,6,2,6],[8,2,6,4],[6,2,2,2]]
 * Output: [[9,9],[8,6]]
 * Explanation: The diagram above shows the original matrix and the generated matrix.
 * Notice that each value in the generated matrix corresponds to the largest value of a contiguous 3 x 3 matrix in grid.
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/07/02/ex2new2.png" style="width: 436px; height: 240px;" />
 * Input: grid = [[1,1,1,1,1],[1,1,1,1,1],[1,1,2,1,1],[1,1,1,1,1],[1,1,1,1,1]]
 * Output: [[2,2,2],[2,2,2],[2,2,2]]
 * Explanation: Notice that the 2 is contained within every contiguous 3 x 3 matrix in grid.
 * 
 *  
 * Constraints:
 * 
 * 	n == grid.length == grid[i].length
 * 	3 <= n <= 100
 * 	1 <= grid[i][j] <= 100
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/largest-local-values-in-a-matrix/
// discuss: https://leetcode.com/problems/largest-local-values-in-a-matrix/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2373() {
    }
}
