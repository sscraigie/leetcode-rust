/**
 * [780] Reaching Points
 *
 * Given four integers sx, sy, tx, and ty, return true if it is possible to convert the point (sx, sy) to the point (tx, ty) through some operations, or false otherwise.
 * The allowed operation on some point (x, y) is to convert it to either (x, x + y) or (x + y, y).
 *  
 * <strong class="example">Example 1:
 * 
 * Input: sx = 1, sy = 1, tx = 3, ty = 5
 * Output: true
 * Explanation:
 * One series of moves that transforms the starting point to the target is:
 * (1, 1) -> (1, 2)
 * (1, 2) -> (3, 2)
 * (3, 2) -> (3, 5)
 * 
 * <strong class="example">Example 2:
 * 
 * Input: sx = 1, sy = 1, tx = 2, ty = 2
 * Output: false
 * 
 * <strong class="example">Example 3:
 * 
 * Input: sx = 1, sy = 1, tx = 1, ty = 1
 * Output: true
 * 
 *  
 * Constraints:
 * 
 * 	1 <= sx, sy, tx, ty <= 10^9
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/reaching-points/
// discuss: https://leetcode.com/problems/reaching-points/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn reaching_points(sx: i32, sy: i32, tx: i32, ty: i32) -> bool {
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_780() {
    }
}
