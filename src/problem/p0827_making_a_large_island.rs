/**
 * [827] Making A Large Island
 *
 * You are given an n x n binary matrix grid. You are allowed to change at most one 0 to be 1.
 * 
 * Return the size of the largest island in grid after applying this operation.
 * 
 * An island is a 4-directionally connected group of 1s.
 * 
 *  
 * <strong class="example">Example 1:
 * 
 * 
 * Input: grid = [[1,0],[0,1]]
 * Output: 3
 * Explanation: Change one 0 to 1 and connect two 1s, then we get an island with area = 3.
 * 
 * 
 * <strong class="example">Example 2:
 * 
 * 
 * Input: grid = [[1,1],[1,0]]
 * Output: 4
 * Explanation: Change the 0 to 1 and make the island bigger, only one island with area = 4.
 * 
 * <strong class="example">Example 3:
 * 
 * 
 * Input: grid = [[1,1],[1,1]]
 * Output: 4
 * Explanation: Can't change any 0 to 1, only one island with area = 4.
 * 
 * 
 *  
 * Constraints:
 * 
 * 
 * 	n == grid.length
 * 	n == grid[i].length
 * 	1 <= n <= 500
 * 	grid[i][j] is either 0 or 1.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/making-a-large-island/
// discuss: https://leetcode.com/problems/making-a-large-island/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn largest_island(grid: Vec<Vec<i32>>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_827() {
    }
}
