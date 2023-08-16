/**
 * [789] Escape The Ghosts
 *
 * You are playing a simplified PAC-MAN game on an infinite 2-D grid. You start at the point [0, 0], and you are given a destination point target = [xtarget, ytarget] that you are trying to get to. There are several ghosts on the map with their starting positions given as a 2D array ghosts, where ghosts[i] = [xi, yi] represents the starting position of the i^th ghost. All inputs are integral coordinates.
 * Each turn, you and all the ghosts may independently choose to either move 1 unit in any of the four cardinal directions: north, east, south, or west, or stay still. All actions happen simultaneously.
 * You escape if and only if you can reach the target before any ghost reaches you. If you reach any square (including the target) at the same time as a ghost, it does not count as an escape.
 * Return true if it is possible to escape regardless of how the ghosts move, otherwise return false.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: ghosts = [[1,0],[0,3]], target = [0,1]
 * Output: true
 * Explanation: You can reach the destination (0, 1) after 1 turn, while the ghosts located at (1, 0) and (0, 3) cannot catch up with you.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: ghosts = [[1,0]], target = [2,0]
 * Output: false
 * Explanation: You need to reach the destination (2, 0), but the ghost at (1, 0) lies between you and the destination.
 * 
 * <strong class="example">Example 3:
 * 
 * Input: ghosts = [[2,0]], target = [1,0]
 * Output: false
 * Explanation: The ghost can reach the target at the same time as you.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= ghosts.length <= 100
 * 	ghosts[i].length == 2
 * 	-10^4 <= xi, yi <= 10^4
 * 	There can be multiple ghosts in the same location.
 * 	target.length == 2
 * 	-10^4 <= xtarget, ytarget <= 10^4
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/escape-the-ghosts/
// discuss: https://leetcode.com/problems/escape-the-ghosts/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn escape_ghosts(ghosts: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_789() {
    }
}
