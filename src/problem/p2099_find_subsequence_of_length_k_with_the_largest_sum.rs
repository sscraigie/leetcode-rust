/**
 * [2099] Find Subsequence of Length K With the Largest Sum
 *
 * You are given an integer array nums and an integer k. You want to find a subsequence of nums of length k that has the largest sum.
 * Return any such subsequence as an integer array of length k.
 * A subsequence is an array that can be derived from another array by deleting some or no elements without changing the order of the remaining elements.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: nums = [2,1,3,3], k = 2
 * Output: [3,3]
 * Explanation:
 * The subsequence has the largest sum of 3 + 3 = 6.
 * <strong class="example">Example 2:
 * 
 * Input: nums = [-1,-2,3,4], k = 3
 * Output: [-1,3,4]
 * Explanation: 
 * The subsequence has the largest sum of -1 + 3 + 4 = 6.
 * 
 * <strong class="example">Example 3:
 * 
 * Input: nums = [3,4,3,3], k = 2
 * Output: [3,4]
 * Explanation:
 * The subsequence has the largest sum of 3 + 4 = 7. 
 * Another possible subsequence is [4, 3].
 * 
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 1000
 * 	-10^5 <= nums[i] <= 10^5
 * 	1 <= k <= nums.length
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-subsequence-of-length-k-with-the-largest-sum/
// discuss: https://leetcode.com/problems/find-subsequence-of-length-k-with-the-largest-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_subsequence(nums: Vec<i32>, k: i32) -> Vec<i32> {
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2099() {
    }
}
