/**
 * [492] Construct the Rectangle
 *
 * A web developer needs to know how to design a web page's size. So, given a specific rectangular web page&rsquo;s area, your job by now is to design a rectangular web page, whose length L and width W satisfy the following requirements:
 * <ol>
 * 	The area of the rectangular web page you designed must equal to the given target area.
 * 	The width W should not be larger than the length L, which means L >= W.
 * 	The difference between length L and width W should be as small as possible.
 * </ol>
 * Return an array [L, W] where L and W are the length and width of the web page you designed in sequence.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: area = 4
 * Output: [2,2]
 * Explanation: The target area is 4, and all the possible ways to construct it are [1,4], [2,2], [4,1]. 
 * But according to requirement 2, [1,4] is illegal; according to requirement 3,  [4,1] is not optimal compared to [2,2]. So the length L is 2, and the width W is 2.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: area = 37
 * Output: [37,1]
 * 
 * <strong class="example">Example 3:
 * 
 * Input: area = 122122
 * Output: [427,286]
 * 
 *  
 * Constraints:
 * 
 * 	1 <= area <= 10^7
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/construct-the-rectangle/
// discuss: https://leetcode.com/problems/construct-the-rectangle/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn construct_rectangle(area: i32) -> Vec<i32> {
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_492() {
    }
}
