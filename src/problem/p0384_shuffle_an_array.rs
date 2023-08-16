/**
 * [384] Shuffle an Array
 *
 * Given an integer array nums, design an algorithm to randomly shuffle the array. All permutations of the array should be equally likely as a result of the shuffling.
 * Implement the Solution class:
 * 
 * 	Solution(int[] nums) Initializes the object with the integer array nums.
 * 	int[] reset() Resets the array to its original configuration and returns it.
 * 	int[] shuffle() Returns a random shuffling of the array.
 * 
 *  
 * <strong class="example">Example 1:
 * 
 * Input
 * ["Solution", "shuffle", "reset", "shuffle"]
 * [[[1, 2, 3]], [], [], []]
 * Output
 * [null, [3, 1, 2], [1, 2, 3], [1, 3, 2]]
 * Explanation
 * Solution solution = new Solution([1, 2, 3]);
 * solution.shuffle();    // Shuffle the array [1,2,3] and return its result.
 *                        // Any permutation of [1,2,3] must be equally likely to be returned.
 *                        // Example: return [3, 1, 2]
 * solution.reset();      // Resets the array back to its original configuration [1,2,3]. Return [1, 2, 3]
 * solution.shuffle();    // Returns the random shuffling of array [1,2,3]. Example: return [1, 3, 2]
 * 
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 50
 * 	-10^6 <= nums[i] <= 10^6
 * 	All the elements of nums are unique.
 * 	At most 10^4 calls in total will be made to reset and shuffle.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/shuffle-an-array/
// discuss: https://leetcode.com/problems/shuffle-an-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

struct Solution {
        false
    }


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {

    fn new(nums: Vec<i32>) -> Self {
        
    }
    
    fn reset(&self) -> Vec<i32> {
        
    }
    
    fn shuffle(&self) -> Vec<i32> {
        
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(nums);
 * let ret_1: Vec<i32> = obj.reset();
 * let ret_2: Vec<i32> = obj.shuffle();
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_384() {
    }
}
