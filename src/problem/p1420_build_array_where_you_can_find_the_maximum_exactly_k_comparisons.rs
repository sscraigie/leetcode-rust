/**
 * [1420] Build Array Where You Can Find The Maximum Exactly K Comparisons
 *
 * You are given three integers n, m and k. Consider the following algorithm to find the maximum element of an array of positive integers:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/04/02/e.png" style="width: 424px; height: 372px;" />
 * You should build the array arr which has the following properties:
 * 
 * 	arr has exactly n integers.
 * 	1 <= arr[i] <= m where (0 <= i < n).
 * 	After applying the mentioned algorithm to arr, the value search_cost is equal to k.
 * 
 * Return the number of ways to build the array arr under the mentioned conditions. As the answer may grow large, the answer must be computed modulo 10^9 + 7.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: n = 2, m = 3, k = 1
 * Output: 6
 * Explanation: The possible arrays are [1, 1], [2, 1], [2, 2], [3, 1], [3, 2] [3, 3]
 * 
 * <strong class="example">Example 2:
 * 
 * Input: n = 5, m = 2, k = 3
 * Output: 0
 * Explanation: There are no possible arrays that satisify the mentioned conditions.
 * 
 * <strong class="example">Example 3:
 * 
 * Input: n = 9, m = 1, k = 1
 * Output: 1
 * Explanation: The only possible array is [1, 1, 1, 1, 1, 1, 1, 1, 1]
 * 
 *  
 * Constraints:
 * 
 * 	1 <= n <= 50
 * 	1 <= m <= 100
 * 	0 <= k <= n
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/build-array-where-you-can-find-the-maximum-exactly-k-comparisons/
// discuss: https://leetcode.com/problems/build-array-where-you-can-find-the-maximum-exactly-k-comparisons/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn num_of_arrays(n: i32, m: i32, k: i32) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1420() {
    }
}
