/**
 * [805] Split Array With Same Average
 *
 * You are given an integer array nums.
 * You should move each element of nums into one of the two arrays A and B such that A and B are non-empty, and average(A) == average(B).
 * Return true if it is possible to achieve that and false otherwise.
 * Note that for an array arr, average(arr) is the sum of all the elements of arr over the length of arr.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: nums = [1,2,3,4,5,6,7,8]
 * Output: true
 * Explanation: We can split the array into [1,4,5,8] and [2,3,6,7], and both of them have an average of 4.5.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: nums = [3,1]
 * Output: false
 * 
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 30
 * 	0 <= nums[i] <= 10^4
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/split-array-with-same-average/
// discuss: https://leetcode.com/problems/split-array-with-same-average/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn split_array_same_average(nums: Vec<i32>) -> bool {
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_805() {
    }
}
