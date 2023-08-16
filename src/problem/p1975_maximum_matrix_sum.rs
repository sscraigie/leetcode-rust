/**
 * [1975] Maximum Matrix Sum
 *
 * You are given an n x n integer matrix. You can do the following operation any number of times:
 * 
 * 	Choose any two adjacent elements of matrix and multiply each of them by -1.
 * 
 * Two elements are considered adjacent if and only if they share a border.
 * Your goal is to maximize the summation of the matrix's elements. Return the maximum sum of the matrix's elements using the operation mentioned above.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/07/16/pc79-q2ex1.png" style="width: 401px; height: 81px;" />
 * Input: matrix = [[1,-1],[-1,1]]
 * Output: 4
 * Explanation: We can follow the following steps to reach sum equals 4:
 * - Multiply the 2 elements in the first row by -1.
 * - Multiply the 2 elements in the first column by -1.
 * 
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/07/16/pc79-q2ex2.png" style="width: 321px; height: 121px;" />
 * Input: matrix = [[1,2,3],[-1,-2,-3],[1,2,3]]
 * Output: 16
 * Explanation: We can follow the following step to reach sum equals 16:
 * - Multiply the 2 last elements in the second row by -1.
 * 
 *  
 * Constraints:
 * 
 * 	n == matrix.length == matrix[i].length
 * 	2 <= n <= 250
 * 	-10^5 <= matrix[i][j] <= 10^5
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-matrix-sum/
// discuss: https://leetcode.com/problems/maximum-matrix-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
        
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1975() {
    }
}
