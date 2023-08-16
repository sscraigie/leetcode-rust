/**
 * [1072] Flip Columns For Maximum Number of Equal Rows
 *
 * You are given an m x n binary matrix matrix.
 * You can choose any number of columns in the matrix and flip every cell in that column (i.e., Change the value of the cell from 0 to 1 or vice versa).
 * Return the maximum number of rows that have all values equal after some number of flips.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: matrix = [[0,1],[1,1]]
 * Output: 1
 * Explanation: After flipping no values, 1 row has all values equal.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: matrix = [[0,1],[1,0]]
 * Output: 2
 * Explanation: After flipping values in the first column, both rows have equal values.
 * 
 * <strong class="example">Example 3:
 * 
 * Input: matrix = [[0,0,0],[0,0,1],[1,1,0]]
 * Output: 2
 * Explanation: After flipping values in the first two columns, the last two rows have equal values.
 * 
 *  
 * Constraints:
 * 
 * 	m == matrix.length
 * 	n == matrix[i].length
 * 	1 <= m, n <= 300
 * 	matrix[i][j] is either 0 or 1.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/flip-columns-for-maximum-number-of-equal-rows/
// discuss: https://leetcode.com/problems/flip-columns-for-maximum-number-of-equal-rows/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1072() {
    }
}
