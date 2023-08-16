/**
 * [2220] Minimum Bit Flips to Convert Number
 *
 * A bit flip of a number x is choosing a bit in the binary representation of x and flipping it from either 0 to 1 or 1 to 0.
 * 
 * 	For example, for x = 7, the binary representation is 111 and we may choose any bit (including any leading zeros not shown) and flip it. We can flip the first bit from the right to get 110, flip the second bit from the right to get 101, flip the fifth bit from the right (a leading zero) to get 10111, etc.
 * 
 * Given two integers start and goal, return the minimum number of bit flips to convert start to goal.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: start = 10, goal = 7
 * Output: 3
 * Explanation: The binary representation of 10 and 7 are 1010 and 0111 respectively. We can convert 10 to 7 in 3 steps:
 * - Flip the first bit from the right: 101<u>0</u> -> 101<u>1</u>.
 * - Flip the third bit from the right: 1<u>0</u>11 -> 1<u>1</u>11.
 * - Flip the fourth bit from the right: <u>1</u>111 -> <u>0</u>111.
 * It can be shown we cannot convert 10 to 7 in less than 3 steps. Hence, we return 3.
 * <strong class="example">Example 2:
 * 
 * Input: start = 3, goal = 4
 * Output: 3
 * Explanation: The binary representation of 3 and 4 are 011 and 100 respectively. We can convert 3 to 4 in 3 steps:
 * - Flip the first bit from the right: 01<u>1</u> -> 01<u>0</u>.
 * - Flip the second bit from the right: 0<u>1</u>0 -> 0<u>0</u>0.
 * - Flip the third bit from the right: <u>0</u>00 -> <u>1</u>00.
 * It can be shown we cannot convert 3 to 4 in less than 3 steps. Hence, we return 3.
 * 
 *  
 * Constraints:
 * 
 * 	0 <= start, goal <= 10^9
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-bit-flips-to-convert-number/
// discuss: https://leetcode.com/problems/minimum-bit-flips-to-convert-number/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_bit_flips(start: i32, goal: i32) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2220() {
    }
}
