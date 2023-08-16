/**
 * [335] Self Crossing
 *
 * You are given an array of integers distance.
 * You start at the point (0, 0) on an X-Y plane, and you move distance[0] meters to the north, then distance[1] meters to the west, distance[2] meters to the south, distance[3] meters to the east, and so on. In other words, after each move, your direction changes counter-clockwise.
 * Return true if your path crosses itself or false if it does not.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/12/21/11.jpg" style="width: 400px; height: 413px;" />
 * Input: distance = [2,1,1,2]
 * Output: true
 * Explanation: The path crosses itself at the point (0, 1).
 * 
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/12/21/22.jpg" style="width: 400px; height: 413px;" />
 * Input: distance = [1,2,3,4]
 * Output: false
 * Explanation: The path does not cross itself at any point.
 * 
 * <strong class="example">Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/12/21/33.jpg" style="width: 400px; height: 413px;" />
 * Input: distance = [1,1,1,2,1]
 * Output: true
 * Explanation: The path crosses itself at the point (0, 0).
 * 
 *  
 * Constraints:
 * 
 * 	1 <= distance.length <= 10^5
 * 	1 <= distance[i] <= 10^5
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/self-crossing/
// discuss: https://leetcode.com/problems/self-crossing/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn is_self_crossing(distance: Vec<i32>) -> bool {
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_335() {
    }
}
