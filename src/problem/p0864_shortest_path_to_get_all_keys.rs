/**
 * [864] Shortest Path to Get All Keys
 *
 * You are given an m x n grid grid where:
 * 
 * 	'.' is an empty cell.
 * 	'#' is a wall.
 * 	'@' is the starting point.
 * 	Lowercase letters represent keys.
 * 	Uppercase letters represent locks.
 * 
 * You start at the starting point and one move consists of walking one space in one of the four cardinal directions. You cannot walk outside the grid, or walk into a wall.
 * If you walk over a key, you can pick it up and you cannot walk over a lock unless you have its corresponding key.
 * For some <font face="monospace">1 <= k <= 6</font>, there is exactly one lowercase and one uppercase letter of the first k letters of the English alphabet in the grid. This means that there is exactly one key for each lock, and one lock for each key; and also that the letters used to represent the keys and locks were chosen in the same order as the English alphabet.
 * Return the lowest number of moves to acquire all keys. If it is impossible, return -1.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/07/23/lc-keys2.jpg" style="width: 404px; height: 245px;" />
 * Input: grid = ["@.a..","###.#","b.A.B"]
 * Output: 8
 * Explanation: Note that the goal is to obtain all the keys not to open all the locks.
 * 
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/07/23/lc-key2.jpg" style="width: 404px; height: 245px;" />
 * Input: grid = ["@..aA","..B#.","....b"]
 * Output: 6
 * 
 * <strong class="example">Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/07/23/lc-keys3.jpg" style="width: 244px; height: 85px;" />
 * Input: grid = ["@Aa"]
 * Output: -1
 * 
 *  
 * Constraints:
 * 
 * 	m == grid.length
 * 	n == grid[i].length
 * 	1 <= m, n <= 30
 * 	grid[i][j] is either an English letter, '.', '#', or '@'. 
 * 	There is exactly one '@' in the grid.
 * 	The number of keys in the grid is in the range [1, 6].
 * 	Each key in the grid is unique.
 * 	Each key in the grid has a matching lock.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/shortest-path-to-get-all-keys/
// discuss: https://leetcode.com/problems/shortest-path-to-get-all-keys/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn shortest_path_all_keys(grid: Vec<String>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_864() {
    }
}
