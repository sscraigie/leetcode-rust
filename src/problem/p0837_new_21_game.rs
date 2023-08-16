/**
 * [837] New 21 Game
 *
 * Alice plays the following game, loosely based on the card game "21".
 * Alice starts with 0 points and draws numbers while she has less than k points. During each draw, she gains an integer number of points randomly from the range [1, maxPts], where maxPts is an integer. Each draw is independent and the outcomes have equal probabilities.
 * Alice stops drawing numbers when she gets k or more points.
 * Return the probability that Alice has n or fewer points.
 * Answers within 10^-5 of the actual answer are considered accepted.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: n = 10, k = 1, maxPts = 10
 * Output: 1.00000
 * Explanation: Alice gets a single card, then stops.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: n = 6, k = 1, maxPts = 10
 * Output: 0.60000
 * Explanation: Alice gets a single card, then stops.
 * In 6 out of 10 possibilities, she is at or below 6 points.
 * 
 * <strong class="example">Example 3:
 * 
 * Input: n = 21, k = 17, maxPts = 10
 * Output: 0.73278
 * 
 *  
 * Constraints:
 * 
 * 	0 <= k <= n <= 10^4
 * 	1 <= maxPts <= 10^4
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/new-21-game/
// discuss: https://leetcode.com/problems/new-21-game/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn new21_game(n: i32, k: i32, max_pts: i32) -> f64 {
        0f64
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_837() {
    }
}
