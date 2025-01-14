/**
 * [598] Range Addition II
 *
 * You are given an m x n matrix M initialized with all 0's and an array of operations ops, where ops[i] = [ai, bi] means M[x][y] should be incremented by one for all 0 <= x < ai and 0 <= y < bi.
 * Count and return the number of maximum integers in the matrix after performing all the operations.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/02/ex1.jpg" style="width: 750px; height: 176px;" />
 * Input: m = 3, n = 3, ops = [[2,2],[3,3]]
 * Output: 4
 * Explanation: The maximum integer in M is 2, and there are four of it in M. So return 4.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: m = 3, n = 3, ops = [[2,2],[3,3],[3,3],[3,3],[2,2],[3,3],[3,3],[3,3],[2,2],[3,3],[3,3],[3,3]]
 * Output: 4
 * 
 * <strong class="example">Example 3:
 * 
 * Input: m = 3, n = 3, ops = []
 * Output: 9
 * 
 *  
 * Constraints:
 * 
 * 	1 <= m, n <= 4 * 10^4
 * 	0 <= ops.length <= 10^4
 * 	ops[i].length == 2
 * 	1 <= ai <= m
 * 	1 <= bi <= n
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/range-addition-ii/
// discuss: https://leetcode.com/problems/range-addition-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_count(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_598() {
    }
}
