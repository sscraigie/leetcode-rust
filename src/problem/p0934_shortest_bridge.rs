/**
 * [934] Shortest Bridge
 *
 * You are given an n x n binary matrix grid where 1 represents land and 0 represents water.
 * An island is a 4-directionally connected group of 1's not connected to any other 1's. There are exactly two islands in grid.
 * You may change 0's to 1's to connect the two islands to form one island.
 * Return the smallest number of 0's you must flip to connect the two islands.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: grid = [[0,1],[1,0]]
 * Output: 1
 * 
 * <strong class="example">Example 2:
 * 
 * Input: grid = [[0,1,0],[0,0,0],[0,0,1]]
 * Output: 2
 * 
 * <strong class="example">Example 3:
 * 
 * Input: grid = [[1,1,1,1,1],[1,0,0,0,1],[1,0,1,0,1],[1,0,0,0,1],[1,1,1,1,1]]
 * Output: 1
 * 
 *  
 * Constraints:
 * 
 * 	n == grid.length == grid[i].length
 * 	2 <= n <= 100
 * 	grid[i][j] is either 0 or 1.
 * 	There are exactly two islands in grid.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/shortest-bridge/
// discuss: https://leetcode.com/problems/shortest-bridge/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn shortest_bridge(grid: Vec<Vec<i32>>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_934() {
    }
}
