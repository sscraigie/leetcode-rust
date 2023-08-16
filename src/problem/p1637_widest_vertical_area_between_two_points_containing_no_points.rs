/**
 * [1637] Widest Vertical Area Between Two Points Containing No Points
 *
 * Given n points on a 2D plane where points[i] = [xi, yi], Return the widest vertical area between two points such that no points are inside the area.
 * A vertical area is an area of fixed-width extending infinitely along the y-axis (i.e., infinite height). The widest vertical area is the one with the maximum width.
 * Note that points on the edge of a vertical area are not considered included in the area.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/19/points3.png" style="width: 276px; height: 371px;" />​
 * Input: points = [[8,7],[9,9],[7,4],[9,7]]
 * Output: 1
 * Explanation: Both the red and the blue area are optimal.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: points = [[3,1],[9,0],[1,0],[1,4],[5,3],[8,8]]
 * Output: 3
 * 
 *  
 * Constraints:
 * 
 * 	n == points.length
 * 	2 <= n <= 10^5
 * 	points[i].length == 2
 * 	0 <= xi, yi <= 10^9
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/widest-vertical-area-between-two-points-containing-no-points/
// discuss: https://leetcode.com/problems/widest-vertical-area-between-two-points-containing-no-points/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_width_of_vertical_area(points: Vec<Vec<i32>>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1637() {
    }
}
