/**
 * [1314] Matrix Block Sum
 *
 * Given a m x n matrix mat and an integer k, return a matrix answer where each answer[i][j] is the sum of all elements mat[r][c] for:
 * 
 * 	i - k <= r <= i + k,
 * 	j - k <= c <= j + k, and
 * 	(r, c) is a valid position in the matrix.
 * 
 *  
 * <strong class="example">Example 1:
 * 
 * Input: mat = [[1,2,3],[4,5,6],[7,8,9]], k = 1
 * Output: [[12,21,16],[27,45,33],[24,39,28]]
 * 
 * <strong class="example">Example 2:
 * 
 * Input: mat = [[1,2,3],[4,5,6],[7,8,9]], k = 2
 * Output: [[45,45,45],[45,45,45],[45,45,45]]
 * 
 *  
 * Constraints:
 * 
 * 	m == mat.length
 * 	n == mat[i].length
 * 	1 <= m, n, k <= 100
 * 	1 <= mat[i][j] <= 100
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/matrix-block-sum/
// discuss: https://leetcode.com/problems/matrix-block-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn matrix_block_sum(mat: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1314() {
    }
}
