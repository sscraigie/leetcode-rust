/**
 * [1861] Rotating the Box
 *
 * You are given an m x n matrix of characters box representing a side-view of a box. Each cell of the box is one of the following:
 * 
 * 
 * 	A stone '#'
 * 	A stationary obstacle '*'
 * 	Empty '.'
 * 
 * 
 * The box is rotated 90 degrees clockwise, causing some of the stones to fall due to gravity. Each stone falls down until it lands on an obstacle, another stone, or the bottom of the box. Gravity does not affect the obstacles' positions, and the inertia from the box's rotation does not affect the stones' horizontal positions.
 * 
 * It is guaranteed that each stone in box rests on an obstacle, another stone, or the bottom of the box.
 * 
 * Return an n x m matrix representing the box after the rotation described above.
 * 
 *  
 * <strong class="example">Example 1:
 * 
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/04/08/rotatingtheboxleetcodewithstones.png" style="width: 300px; height: 150px;" />
 * 
 * 
 * Input: box = [["#",".","#"]]
 * Output: [["."],
 *          ["#"],
 *          ["#"]]
 * 
 * 
 * <strong class="example">Example 2:
 * 
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/04/08/rotatingtheboxleetcode2withstones.png" style="width: 375px; height: 195px;" />
 * 
 * 
 * Input: box = [["#",".","*","."],
 *               ["#","#","*","."]]
 * Output: [["#","."],
 *          ["#","#"],
 *          ["*","*"],
 *          [".","."]]
 * 
 * 
 * <strong class="example">Example 3:
 * 
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/04/08/rotatingtheboxleetcode3withstone.png" style="width: 400px; height: 218px;" />
 * 
 * 
 * Input: box = [["#","#","*",".","*","."],
 *               ["#","#","#","*",".","."],
 *               ["#","#","#",".","#","."]]
 * Output: [[".","#","#"],
 *          [".","#","#"],
 *          ["#","#","*"],
 *          ["#","*","."],
 *          ["#",".","*"],
 *          ["#",".","."]]
 * 
 * 
 *  
 * Constraints:
 * 
 * 
 * 	m == box.length
 * 	n == box[i].length
 * 	1 <= m, n <= 500
 * 	box[i][j] is either '#', '*', or '.'.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/rotating-the-box/
// discuss: https://leetcode.com/problems/rotating-the-box/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn rotate_the_box(box: Vec<Vec<char>>) -> Vec<Vec<char>> {
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1861() {
    }
}
