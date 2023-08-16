/**
 * [374] Guess Number Higher or Lower
 *
 * We are playing the Guess Game. The game is as follows:
 * I pick a number from 1 to n. You have to guess which number I picked.
 * Every time you guess wrong, I will tell you whether the number I picked is higher or lower than your guess.
 * You call a pre-defined API int guess(int num), which returns three possible results:
 * 
 * 	-1: Your guess is higher than the number I picked (i.e. num > pick).
 * 	1: Your guess is lower than the number I picked (i.e. num < pick).
 * 	0: your guess is equal to the number I picked (i.e. num == pick).
 * 
 * Return the number that I picked.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: n = 10, pick = 6
 * Output: 6
 * 
 * <strong class="example">Example 2:
 * 
 * Input: n = 1, pick = 1
 * Output: 1
 * 
 * <strong class="example">Example 3:
 * 
 * Input: n = 2, pick = 1
 * Output: 1
 * 
 *  
 * Constraints:
 * 
 * 	1 <= n <= 2^31 - 1
 * 	1 <= pick <= n
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/guess-number-higher-or-lower/
// discuss: https://leetcode.com/problems/guess-number-higher-or-lower/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

/** 
 * Forward declaration of guess API.
 * @param  num   your guess
 * @return 	     -1 if num is higher than the picked number
 *			      1 if num is lower than the picked number
 *               otherwise return 0
 * unsafe fn guess(num: i32) -> i32 {}
 */

impl Solution {
    unsafe fn guessNumber(n: i32) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_374() {
    }
}
