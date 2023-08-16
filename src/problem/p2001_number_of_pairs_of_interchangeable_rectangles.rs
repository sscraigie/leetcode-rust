/**
 * [2001] Number of Pairs of Interchangeable Rectangles
 *
 * You are given n rectangles represented by a 0-indexed 2D integer array rectangles, where rectangles[i] = [widthi, heighti] denotes the width and height of the i^th rectangle.
 * Two rectangles i and j (i < j) are considered interchangeable if they have the same width-to-height ratio. More formally, two rectangles are interchangeable if widthi/heighti == widthj/heightj (using decimal division, not integer division).
 * Return the number of pairs of interchangeable rectangles in rectangles.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: rectangles = [[4,8],[3,6],[10,20],[15,30]]
 * Output: 6
 * Explanation: The following are the interchangeable pairs of rectangles by index (0-indexed):
 * - Rectangle 0 with rectangle 1: 4/8 == 3/6.
 * - Rectangle 0 with rectangle 2: 4/8 == 10/20.
 * - Rectangle 0 with rectangle 3: 4/8 == 15/30.
 * - Rectangle 1 with rectangle 2: 3/6 == 10/20.
 * - Rectangle 1 with rectangle 3: 3/6 == 15/30.
 * - Rectangle 2 with rectangle 3: 10/20 == 15/30.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: rectangles = [[4,5],[7,8]]
 * Output: 0
 * Explanation: There are no interchangeable pairs of rectangles.
 * 
 *  
 * Constraints:
 * 
 * 	n == rectangles.length
 * 	1 <= n <= 10^5
 * 	rectangles[i].length == 2
 * 	1 <= widthi, heighti <= 10^5
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-pairs-of-interchangeable-rectangles/
// discuss: https://leetcode.com/problems/number-of-pairs-of-interchangeable-rectangles/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn interchangeable_rectangles(rectangles: Vec<Vec<i32>>) -> i64 {
        
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2001() {
    }
}
