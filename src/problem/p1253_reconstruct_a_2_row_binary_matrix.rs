/**
 * [1253] Reconstruct a 2-Row Binary Matrix
 *
 * Given the following details of a matrix with n columns and 2 rows :
 * 
 * 	The matrix is a binary matrix, which means each element in the matrix can be 0 or 1.
 * 	The sum of elements of the 0-th(upper) row is given as upper.
 * 	The sum of elements of the 1-st(lower) row is given as lower.
 * 	The sum of elements in the i-th column(0-indexed) is colsum[i], where colsum is given as an integer array with length n.
 * 
 * Your task is to reconstruct the matrix with upper, lower and colsum.
 * Return it as a 2-D integer array.
 * If there are more than one valid solution, any of them will be accepted.
 * If no valid solution exists, return an empty 2-D array.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: upper = 2, lower = 1, colsum = [1,1,1]
 * Output: [[1,1,0],[0,0,1]]
 * Explanation: [[1,0,1],[0,1,0]], and [[0,1,1],[1,0,0]] are also correct answers.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: upper = 2, lower = 3, colsum = [2,2,1,1]
 * Output: []
 * 
 * <strong class="example">Example 3:
 * 
 * Input: upper = 5, lower = 5, colsum = [2,1,2,0,1,0,1,2,0,1]
 * Output: [[1,1,1,0,1,0,0,1,0,0],[1,0,1,0,0,0,1,1,0,1]]
 * 
 *  
 * Constraints:
 * 
 * 	1 <= colsum.length <= 10^5
 * 	0 <= upper, lower <= colsum.length
 * 	0 <= colsum[i] <= 2
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/reconstruct-a-2-row-binary-matrix/
// discuss: https://leetcode.com/problems/reconstruct-a-2-row-binary-matrix/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn reconstruct_matrix(upper: i32, lower: i32, colsum: Vec<i32>) -> Vec<Vec<i32>> {
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1253() {
    }
}
