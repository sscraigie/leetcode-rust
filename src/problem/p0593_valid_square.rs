/**
 * [593] Valid Square
 *
 * Given the coordinates of four points in 2D space p1, p2, p3 and p4, return true if the four points construct a square.
 * The coordinate of a point pi is represented as [xi, yi]. The input is not given in any order.
 * A valid square has four equal sides with positive length and four equal angles (90-degree angles).
 *  
 * <strong class="example">Example 1:
 * 
 * Input: p1 = [0,0], p2 = [1,1], p3 = [1,0], p4 = [0,1]
 * Output: true
 * 
 * <strong class="example">Example 2:
 * 
 * Input: p1 = [0,0], p2 = [1,1], p3 = [1,0], p4 = [0,12]
 * Output: false
 * 
 * <strong class="example">Example 3:
 * 
 * Input: p1 = [1,0], p2 = [-1,0], p3 = [0,1], p4 = [0,-1]
 * Output: true
 * 
 *  
 * Constraints:
 * 
 * 	p1.length == p2.length == p3.length == p4.length == 2
 * 	-10^4 <= xi, yi <= 10^4
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/valid-square/
// discuss: https://leetcode.com/problems/valid-square/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn valid_square(p1: Vec<i32>, p2: Vec<i32>, p3: Vec<i32>, p4: Vec<i32>) -> bool {
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_593() {
    }
}
