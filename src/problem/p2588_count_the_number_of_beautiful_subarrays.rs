/**
 * [2588] Count the Number of Beautiful Subarrays
 *
 * You are given a 0-indexed integer array nums. In one operation, you can:
 * 
 * 	Choose two different indices i and j such that 0 <= i, j < nums.length.
 * 	Choose a non-negative integer k such that the k^th bit (0-indexed) in the binary representation of nums[i] and nums[j] is 1.
 * 	Subtract 2^k from nums[i] and nums[j].
 * 
 * A subarray is beautiful if it is possible to make all of its elements equal to 0 after applying the above operation any number of times.
 * Return the number of beautiful subarrays in the array nums.
 * A subarray is a contiguous non-empty sequence of elements within an array.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: nums = [4,3,1,2,4]
 * Output: 2
 * Explanation: There are 2 beautiful subarrays in nums: [4,<u>3,1,2</u>,4] and [<u>4,3,1,2,4</u>].
 * - We can make all elements in the subarray [3,1,2] equal to 0 in the following way:
 *   - Choose [<u>3</u>, 1, <u>2</u>] and k = 1. Subtract 2^1 from both numbers. The subarray becomes [1, 1, 0].
 *   - Choose [<u>1</u>, <u>1</u>, 0] and k = 0. Subtract 2^0 from both numbers. The subarray becomes [0, 0, 0].
 * - We can make all elements in the subarray [4,3,1,2,4] equal to 0 in the following way:
 *   - Choose [<u>4</u>, 3, 1, 2, <u>4</u>] and k = 2. Subtract 2^2 from both numbers. The subarray becomes [0, 3, 1, 2, 0].
 *   - Choose [0, <u>3</u>, <u>1</u>, 2, 0] and k = 0. Subtract 2^0 from both numbers. The subarray becomes [0, 2, 0, 2, 0].
 *   - Choose [0, <u>2</u>, 0, <u>2</u>, 0] and k = 1. Subtract 2^1 from both numbers. The subarray becomes [0, 0, 0, 0, 0].
 * 
 * <strong class="example">Example 2:
 * 
 * Input: nums = [1,10,4]
 * Output: 0
 * Explanation: There are no beautiful subarrays in nums.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 10^5
 * 	0 <= nums[i] <= 10^6
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-the-number-of-beautiful-subarrays/
// discuss: https://leetcode.com/problems/count-the-number-of-beautiful-subarrays/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn beautiful_subarrays(nums: Vec<i32>) -> i64 {
        
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2588() {
    }
}
