/**
 * [1566] Detect Pattern of Length M Repeated K or More Times
 *
 * Given an array of positive integers arr, find a pattern of length m that is repeated k or more times.
 * A pattern is a subarray (consecutive sub-sequence) that consists of one or more values, repeated multiple times consecutively without overlapping. A pattern is defined by its length and the number of repetitions.
 * Return true if there exists a pattern of length m that is repeated k or more times, otherwise return false.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: arr = [1,2,4,4,4,4], m = 1, k = 3
 * Output: true
 * Explanation: The pattern (4) of length 1 is repeated 4 consecutive times. Notice that pattern can be repeated k or more times but not less.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: arr = [1,2,1,2,1,1,1,3], m = 2, k = 2
 * Output: true
 * Explanation: The pattern (1,2) of length 2 is repeated 2 consecutive times. Another valid pattern (2,1) is also repeated 2 times.
 * 
 * <strong class="example">Example 3:
 * 
 * Input: arr = [1,2,1,2,1,3], m = 2, k = 3
 * Output: false
 * Explanation: The pattern (1,2) is of length 2 but is repeated only 2 times. There is no pattern of length 2 that is repeated 3 or more times.
 * 
 *  
 * Constraints:
 * 
 * 	2 <= arr.length <= 100
 * 	1 <= arr[i] <= 100
 * 	1 <= m <= 100
 * 	2 <= k <= 100
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/detect-pattern-of-length-m-repeated-k-or-more-times/
// discuss: https://leetcode.com/problems/detect-pattern-of-length-m-repeated-k-or-more-times/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn contains_pattern(arr: Vec<i32>, m: i32, k: i32) -> bool {
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1566() {
    }
}
