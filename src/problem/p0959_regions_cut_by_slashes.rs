/**
 * [959] Regions Cut By Slashes
 *
 * An n x n grid is composed of 1 x 1 squares where each 1 x 1 square consists of a '/', '\', or blank space ' '. These characters divide the square into contiguous regions.
 * Given the grid grid represented as a string array, return the number of regions.
 * Note that backslash characters are escaped, so a '\' is represented as '\\'.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2018/12/15/1.png" style="width: 200px; height: 200px;" />
 * Input: grid = [" /","/ "]
 * Output: 2
 * 
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2018/12/15/2.png" style="width: 200px; height: 198px;" />
 * Input: grid = [" /","  "]
 * Output: 1
 * 
 * <strong class="example">Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2018/12/15/4.png" style="width: 200px; height: 200px;" />
 * Input: grid = ["/\\","\\/"]
 * Output: 5
 * Explanation: Recall that because \ characters are escaped, "\\/" refers to \/, and "/\\" refers to /\.
 * 
 *  
 * Constraints:
 * 
 * 	n == grid.length == grid[i].length
 * 	1 <= n <= 30
 * 	grid[i][j] is either '/', '\', or ' '.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/regions-cut-by-slashes/
// discuss: https://leetcode.com/problems/regions-cut-by-slashes/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn regions_by_slashes(grid: Vec<String>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_959() {
    }
}
