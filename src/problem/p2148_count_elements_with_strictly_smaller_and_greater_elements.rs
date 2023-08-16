/**
 * [2148] Count Elements With Strictly Smaller and Greater Elements 
 *
 * Given an integer array nums, return the number of elements that have both a strictly smaller and a strictly greater element appear in nums.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: nums = [11,7,2,15]
 * Output: 2
 * Explanation: The element 7 has the element 2 strictly smaller than it and the element 11 strictly greater than it.
 * Element 11 has element 7 strictly smaller than it and element 15 strictly greater than it.
 * In total there are 2 elements having both a strictly smaller and a strictly greater element appear in nums.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: nums = [-3,3,3,90]
 * Output: 2
 * Explanation: The element 3 has the element -3 strictly smaller than it and the element 90 strictly greater than it.
 * Since there are two elements with the value 3, in total there are 2 elements having both a strictly smaller and a strictly greater element appear in nums.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 100
 * 	-10^5 <= nums[i] <= 10^5
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-elements-with-strictly-smaller-and-greater-elements/
// discuss: https://leetcode.com/problems/count-elements-with-strictly-smaller-and-greater-elements/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_elements(nums: Vec<i32>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2148() {
    }
}