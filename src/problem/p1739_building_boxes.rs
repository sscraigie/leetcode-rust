/**
 * [1739] Building Boxes
 *
 * You have a cubic storeroom where the width, length, and height of the room are all equal to n units. You are asked to place n boxes in this room where each box is a cube of unit side length. There are however some rules to placing the boxes:
 * 
 * 	You can place the boxes anywhere on the floor.
 * 	If box x is placed on top of the box y, then each side of the four vertical sides of the box y must either be adjacent to another box or to a wall.
 * 
 * Given an integer n, return the minimum possible number of boxes touching the floor.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/04/3-boxes.png" style="width: 135px; height: 143px;" />
 * 
 * Input: n = 3
 * Output: 3
 * Explanation: The figure above is for the placement of the three boxes.
 * These boxes are placed in the corner of the room, where the corner is on the left side.
 * 
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/04/4-boxes.png" style="width: 135px; height: 179px;" />
 * 
 * Input: n = 4
 * Output: 3
 * Explanation: The figure above is for the placement of the four boxes.
 * These boxes are placed in the corner of the room, where the corner is on the left side.
 * 
 * <strong class="example">Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/04/10-boxes.png" style="width: 271px; height: 257px;" />
 * 
 * Input: n = 10
 * Output: 6
 * Explanation: The figure above is for the placement of the ten boxes.
 * These boxes are placed in the corner of the room, where the corner is on the back side.
 *  
 * Constraints:
 * 
 * 	1 <= n <= 10^9
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/building-boxes/
// discuss: https://leetcode.com/problems/building-boxes/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn minimum_boxes(n: i32) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1739() {
    }
}
