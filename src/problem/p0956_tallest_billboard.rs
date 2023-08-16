/**
 * [956] Tallest Billboard
 *
 * You are installing a billboard and want it to have the largest height. The billboard will have two steel supports, one on each side. Each steel support must be an equal height.
 * You are given a collection of rods that can be welded together. For example, if you have rods of lengths 1, 2, and 3, you can weld them together to make a support of length 6.
 * Return the largest possible height of your billboard installation. If you cannot support the billboard, return 0.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: rods = [1,2,3,6]
 * Output: 6
 * Explanation: We have two disjoint subsets {1,2,3} and {6}, which have the same sum = 6.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: rods = [1,2,3,4,5,6]
 * Output: 10
 * Explanation: We have two disjoint subsets {2,3,5} and {4,6}, which have the same sum = 10.
 * 
 * <strong class="example">Example 3:
 * 
 * Input: rods = [1,2]
 * Output: 0
 * Explanation: The billboard cannot be supported, so we return 0.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= rods.length <= 20
 * 	1 <= rods[i] <= 1000
 * 	sum(rods[i]) <= 5000
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/tallest-billboard/
// discuss: https://leetcode.com/problems/tallest-billboard/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn tallest_billboard(rods: Vec<i32>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_956() {
    }
}
