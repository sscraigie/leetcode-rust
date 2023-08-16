/**
 * [1010] Pairs of Songs With Total Durations Divisible by 60
 *
 * You are given a list of songs where the i^th song has a duration of time[i] seconds.
 * Return the number of pairs of songs for which their total duration in seconds is divisible by 60. Formally, we want the number of indices i, j such that i < j with (time[i] + time[j]) % 60 == 0.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: time = [30,20,150,100,40]
 * Output: 3
 * Explanation: Three pairs have a total duration divisible by 60:
 * (time[0] = 30, time[2] = 150): total duration 180
 * (time[1] = 20, time[3] = 100): total duration 120
 * (time[1] = 20, time[4] = 40): total duration 60
 * 
 * <strong class="example">Example 2:
 * 
 * Input: time = [60,60,60]
 * Output: 3
 * Explanation: All three pairs have a total duration of 120, which is divisible by 60.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= time.length <= 6 * 10^4
 * 	1 <= time[i] <= 500
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/pairs-of-songs-with-total-durations-divisible-by-60/
// discuss: https://leetcode.com/problems/pairs-of-songs-with-total-durations-divisible-by-60/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn num_pairs_divisible_by60(time: Vec<i32>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1010() {
    }
}
