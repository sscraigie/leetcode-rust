/**
 * [120] Triangle
 *
 * Given a triangle array, return the minimum path sum from top to bottom.
 * For each step, you may move to an adjacent number of the row below. More formally, if you are on index i on the current row, you may move to either index i or index i + 1 on the next row.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: triangle = [[2],[3,4],[6,5,7],[4,1,8,3]]
 * Output: 11
 * Explanation: The triangle looks like:
 *    <u>2</u>
 *   <u>3</u> 4
 *  6 <u>5</u> 7
 * 4 <u>1</u> 8 3
 * The minimum path sum from top to bottom is 2 + 3 + 5 + 1 = 11 (underlined above).
 * 
 * <strong class="example">Example 2:
 * 
 * Input: triangle = [[-10]]
 * Output: -10
 * 
 *  
 * Constraints:
 * 
 * 	1 <= triangle.length <= 200
 * 	triangle[0].length == 1
 * 	triangle[i].length == triangle[i - 1].length + 1
 * 	-10^4 <= triangle[i][j] <= 10^4
 * 
 *  
 * Follow up: Could you do this using only O(n) extra space, where n is the total number of rows in the triangle?
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/triangle/
// discuss: https://leetcode.com/problems/triangle/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_120() {
    }
}
