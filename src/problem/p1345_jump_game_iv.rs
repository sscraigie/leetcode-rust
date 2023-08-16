/**
 * [1345] Jump Game IV
 *
 * Given an array of integers arr, you are initially positioned at the first index of the array.
 * In one step you can jump from index i to index:
 * 
 * 	i + 1 where: i + 1 < arr.length.
 * 	i - 1 where: i - 1 >= 0.
 * 	j where: arr[i] == arr[j] and i != j.
 * 
 * Return the minimum number of steps to reach the last index of the array.
 * Notice that you can not jump outside of the array at any time.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: arr = [100,-23,-23,404,100,23,23,23,3,404]
 * Output: 3
 * Explanation: You need three jumps from index 0 --> 4 --> 3 --> 9. Note that index 9 is the last index of the array.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: arr = [7]
 * Output: 0
 * Explanation: Start index is the last index. You do not need to jump.
 * 
 * <strong class="example">Example 3:
 * 
 * Input: arr = [7,6,9,6,9,6,9,7]
 * Output: 1
 * Explanation: You can jump directly from index 0 to index 7 which is last index of the array.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= arr.length <= 5 * 10^4
 * 	-10^8 <= arr[i] <= 10^8
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/jump-game-iv/
// discuss: https://leetcode.com/problems/jump-game-iv/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1345() {
    }
}
