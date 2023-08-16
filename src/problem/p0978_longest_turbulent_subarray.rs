/**
 * [978] Longest Turbulent Subarray
 *
 * Given an integer array arr, return the length of a maximum size turbulent subarray of arr.
 * A subarray is turbulent if the comparison sign flips between each adjacent pair of elements in the subarray.
 * More formally, a subarray [arr[i], arr[i + 1], ..., arr[j]] of arr is said to be turbulent if and only if:
 * 
 * 	For i <= k < j:
 * 	
 * 		arr[k] > arr[k + 1] when k is odd, and
 * 		arr[k] < arr[k + 1] when k is even.
 * 	
 * 	
 * 	Or, for i <= k < j:
 * 	
 * 		arr[k] > arr[k + 1] when k is even, and
 * 		arr[k] < arr[k + 1] when k is odd.
 * 	
 * 	
 * 
 *  
 * <strong class="example">Example 1:
 * 
 * Input: arr = [9,4,2,10,7,8,8,1,9]
 * Output: 5
 * Explanation: arr[1] > arr[2] < arr[3] > arr[4] < arr[5]
 * 
 * <strong class="example">Example 2:
 * 
 * Input: arr = [4,8,12,16]
 * Output: 2
 * 
 * <strong class="example">Example 3:
 * 
 * Input: arr = [100]
 * Output: 1
 * 
 *  
 * Constraints:
 * 
 * 	1 <= arr.length <= 4 * 10^4
 * 	0 <= arr[i] <= 10^9
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-turbulent-subarray/
// discuss: https://leetcode.com/problems/longest-turbulent-subarray/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_turbulence_size(arr: Vec<i32>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_978() {
    }
}
