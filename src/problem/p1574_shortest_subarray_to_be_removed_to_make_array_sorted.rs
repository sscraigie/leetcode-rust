/**
 * [1574] Shortest Subarray to be Removed to Make Array Sorted
 *
 * Given an integer array arr, remove a subarray (can be empty) from arr such that the remaining elements in arr are non-decreasing.
 * Return the length of the shortest subarray to remove.
 * A subarray is a contiguous subsequence of the array.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: arr = [1,2,3,10,4,2,3,5]
 * Output: 3
 * Explanation: The shortest subarray we can remove is [10,4,2] of length 3. The remaining elements after that will be [1,2,3,3,5] which are sorted.
 * Another correct solution is to remove the subarray [3,10,4].
 * 
 * <strong class="example">Example 2:
 * 
 * Input: arr = [5,4,3,2,1]
 * Output: 4
 * Explanation: Since the array is strictly decreasing, we can only keep a single element. Therefore we need to remove a subarray of length 4, either [5,4,3,2] or [4,3,2,1].
 * 
 * <strong class="example">Example 3:
 * 
 * Input: arr = [1,2,3]
 * Output: 0
 * Explanation: The array is already non-decreasing. We do not need to remove any elements.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= arr.length <= 10^5
 * 	0 <= arr[i] <= 10^9
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/shortest-subarray-to-be-removed-to-make-array-sorted/
// discuss: https://leetcode.com/problems/shortest-subarray-to-be-removed-to-make-array-sorted/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1574() {
    }
}
