/**
 * [363] Max Sum of Rectangle No Larger Than K
 *
 * Given an m x n matrix matrix and an integer k, return the max sum of a rectangle in the matrix such that its sum is no larger than k.
 * It is guaranteed that there will be a rectangle with a sum no larger than k.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/18/sum-grid.jpg" style="width: 255px; height: 176px;" />
 * Input: matrix = [[1,0,1],[0,-2,3]], k = 2
 * Output: 2
 * Explanation: Because the sum of the blue rectangle [[0, 1], [-2, 3]] is 2, and 2 is the max number no larger than k (k = 2).
 * 
 * <strong class="example">Example 2:
 * 
 * Input: matrix = [[2,2,-1]], k = 3
 * Output: 3
 * 
 *  
 * Constraints:
 * 
 * 	m == matrix.length
 * 	n == matrix[i].length
 * 	1 <= m, n <= 100
 * 	-100 <= matrix[i][j] <= 100
 * 	-10^5 <= k <= 10^5
 * 
 *  
 * Follow up: What if the number of rows is much larger than the number of columns?
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/max-sum-of-rectangle-no-larger-than-k/
// discuss: https://leetcode.com/problems/max-sum-of-rectangle-no-larger-than-k/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_363() {
    }
}
