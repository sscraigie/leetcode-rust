/**
 * [307] Range Sum Query - Mutable
 *
 * Given an integer array nums, handle multiple queries of the following types:
 * <ol>
 * 	Update the value of an element in nums.
 * 	Calculate the sum of the elements of nums between indices left and right inclusive where left <= right.
 * </ol>
 * Implement the NumArray class:
 * 
 * 	NumArray(int[] nums) Initializes the object with the integer array nums.
 * 	void update(int index, int val) Updates the value of nums[index] to be val.
 * 	int sumRange(int left, int right) Returns the sum of the elements of nums between indices left and right inclusive (i.e. nums[left] + nums[left + 1] + ... + nums[right]).
 * 
 *  
 * <strong class="example">Example 1:
 * 
 * Input
 * ["NumArray", "sumRange", "update", "sumRange"]
 * [[[1, 3, 5]], [0, 2], [1, 2], [0, 2]]
 * Output
 * [null, 9, null, 8]
 * Explanation
 * NumArray numArray = new NumArray([1, 3, 5]);
 * numArray.sumRange(0, 2); // return 1 + 3 + 5 = 9
 * numArray.update(1, 2);   // nums = [1, 2, 5]
 * numArray.sumRange(0, 2); // return 1 + 2 + 5 = 8
 * 
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 3 * 10^4
 * 	-100 <= nums[i] <= 100
 * 	0 <= index < nums.length
 * 	-100 <= val <= 100
 * 	0 <= left <= right < nums.length
 * 	At most 3 * 10^4 calls will be made to update and sumRange.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/range-sum-query-mutable/
// discuss: https://leetcode.com/problems/range-sum-query-mutable/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

struct NumArray {
        vec![]
    }


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {

    fn new(nums: Vec<i32>) -> Self {
        
    }
    
    fn update(&self, index: i32, val: i32) {
        
    }
    
    fn sum_range(&self, left: i32, right: i32) -> i32 {
        
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * obj.update(index, val);
 * let ret_2: i32 = obj.sum_range(left, right);
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_307() {
    }
}
