/**
 * [632] Smallest Range Covering Elements from K Lists
 *
 * You have k lists of sorted integers in non-decreasing order. Find the smallest range that includes at least one number from each of the k lists.
 * We define the range [a, b] is smaller than range [c, d] if b - a < d - c or a < c if b - a == d - c.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: nums = [[4,10,15,24,26],[0,9,12,20],[5,18,22,30]]
 * Output: [20,24]
 * Explanation: 
 * List 1: [4, 10, 15, 24,26], 24 is in range [20,24].
 * List 2: [0, 9, 12, 20], 20 is in range [20,24].
 * List 3: [5, 18, 22, 30], 22 is in range [20,24].
 * 
 * <strong class="example">Example 2:
 * 
 * Input: nums = [[1,2,3],[1,2,3],[1,2,3]]
 * Output: [1,1]
 * 
 *  
 * Constraints:
 * 
 * 	nums.length == k
 * 	1 <= k <= 3500
 * 	1 <= nums[i].length <= 50
 * 	-10^5 <= nums[i][j] <= 10^5
 * 	nums[i] is sorted in non-decreasing order.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/smallest-range-covering-elements-from-k-lists/
// discuss: https://leetcode.com/problems/smallest-range-covering-elements-from-k-lists/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_632() {
    }
}
