/**
 * [2428] Maximum Sum of an Hourglass
 *
 * You are given an m x n integer matrix grid.
 * We define an hourglass as a part of the matrix with the following form:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/08/21/img.jpg" style="width: 243px; height: 243px;" />
 * Return the maximum sum of the elements of an hourglass.
 * Note that an hourglass cannot be rotated and must be entirely contained within the matrix.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/08/21/1.jpg" style="width: 323px; height: 323px;" />
 * Input: grid = [[6,2,1,3],[4,2,1,5],[9,2,8,7],[4,1,2,9]]
 * Output: 30
 * Explanation: The cells shown above represent the hourglass with the maximum sum: 6 + 2 + 1 + 2 + 9 + 2 + 8 = 30.
 * 
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/08/21/2.jpg" style="width: 243px; height: 243px;" />
 * Input: grid = [[1,2,3],[4,5,6],[7,8,9]]
 * Output: 35
 * Explanation: There is only one hourglass in the matrix, with the sum: 1 + 2 + 3 + 5 + 7 + 8 + 9 = 35.
 * 
 *  
 * Constraints:
 * 
 * 	m == grid.length
 * 	n == grid[i].length
 * 	3 <= m, n <= 150
 * 	0 <= grid[i][j] <= 10^6
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-sum-of-an-hourglass/
// discuss: https://leetcode.com/problems/maximum-sum-of-an-hourglass/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_sum(grid: Vec<Vec<i32>>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2428() {
    }
}
