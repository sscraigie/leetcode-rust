/**
 * [754] Reach a Number
 *
 * You are standing at position 0 on an infinite number line. There is a destination at position target.
 * You can make some number of moves numMoves so that:
 * 
 * 	On each move, you can either go left or right.
 * 	During the i^th move (starting from i == 1 to i == numMoves), you take i steps in the chosen direction.
 * 
 * Given the integer target, return the minimum number of moves required (i.e., the minimum numMoves) to reach the destination.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: target = 2
 * Output: 3
 * Explanation:
 * On the 1^st move, we step from 0 to 1 (1 step).
 * On the 2^nd move, we step from 1 to -1 (2 steps).
 * On the 3^rd move, we step from -1 to 2 (3 steps).
 * 
 * <strong class="example">Example 2:
 * 
 * Input: target = 3
 * Output: 2
 * Explanation:
 * On the 1^st move, we step from 0 to 1 (1 step).
 * On the 2^nd move, we step from 1 to 3 (2 steps).
 * 
 *  
 * Constraints:
 * 
 * 	-10^9 <= target <= 10^9
 * 	target != 0
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/reach-a-number/
// discuss: https://leetcode.com/problems/reach-a-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn reach_number(target: i32) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_754() {
    }
}
