/**
 * [2543] Check if Point Is Reachable
 *
 * There exists an infinitely large grid. You are currently at point (1, 1), and you need to reach the point (targetX, targetY) using a finite number of steps.
 * In one step, you can move from point (x, y) to any one of the following points:
 * 
 * 	(x, y - x)
 * 	(x - y, y)
 * 	(2 * x, y)
 * 	(x, 2 * y)
 * 
 * Given two integers targetX and targetY representing the X-coordinate and Y-coordinate of your final position, return true if you can reach the point from (1, 1) using some number of steps, and false otherwise.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: targetX = 6, targetY = 9
 * Output: false
 * Explanation: It is impossible to reach (6,9) from (1,1) using any sequence of moves, so false is returned.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: targetX = 4, targetY = 7
 * Output: true
 * Explanation: You can follow the path (1,1) -> (1,2) -> (1,4) -> (1,8) -> (1,7) -> (2,7) -> (4,7).
 * 
 *  
 * Constraints:
 * 
 * 	1 <= targetX, targetY <= 10^9
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/check-if-point-is-reachable/
// discuss: https://leetcode.com/problems/check-if-point-is-reachable/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_reachable(target_x: i32, target_y: i32) -> bool {
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2543() {
    }
}
