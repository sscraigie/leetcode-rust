/**
 * [1901] Find a Peak Element II
 *
 * A peak element in a 2D grid is an element that is strictly greater than all of its adjacent neighbors to the left, right, top, and bottom.
 * Given a 0-indexed m x n matrix mat where no two adjacent cells are equal, find any peak element mat[i][j] and return the length 2 array [i,j].
 * You may assume that the entire matrix is surrounded by an outer perimeter with the value -1 in each cell.
 * You must write an algorithm that runs in O(m log(n)) or O(n log(m)) time.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/06/08/1.png" style="width: 206px; height: 209px;" />
 * 
 * Input: mat = [[1,4],[3,2]]
 * Output: [0,1]
 * Explanation: Both 3 and 4 are peak elements so [1,0] and [0,1] are both acceptable answers.
 * 
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/06/07/3.png" style="width: 254px; height: 257px;" />
 * 
 * Input: mat = [[10,20,15],[21,30,14],[7,16,32]]
 * Output: [1,1]
 * Explanation: Both 30 and 32 are peak elements so [1,1] and [2,2] are both acceptable answers.
 * 
 *  
 * Constraints:
 * 
 * 	m == mat.length
 * 	n == mat[i].length
 * 	1 <= m, n <= 500
 * 	1 <= mat[i][j] <= 10^5
 * 	No two adjacent cells are equal.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-a-peak-element-ii/
// discuss: https://leetcode.com/problems/find-a-peak-element-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_peak_grid(mat: Vec<Vec<i32>>) -> Vec<i32> {
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1901() {
    }
}