/**
 * [1200] Minimum Absolute Difference
 *
 * Given an array of distinct integers arr, find all pairs of elements with the minimum absolute difference of any two elements.
 * Return a list of pairs in ascending order(with respect to pairs), each pair [a, b] follows
 * 
 * 	a, b are from arr
 * 	a < b
 * 	b - a equals to the minimum absolute difference of any two elements in arr
 * 
 *  
 * <strong class="example">Example 1:
 * 
 * Input: arr = [4,2,1,3]
 * Output: [[1,2],[2,3],[3,4]]
 * Explanation: The minimum absolute difference is 1. List all pairs with difference equal to 1 in ascending order.
 * <strong class="example">Example 2:
 * 
 * Input: arr = [1,3,6,10,15]
 * Output: [[1,3]]
 * 
 * <strong class="example">Example 3:
 * 
 * Input: arr = [3,8,-10,23,19,-4,-14,27]
 * Output: [[-14,-10],[19,23],[23,27]]
 * 
 *  
 * Constraints:
 * 
 * 	2 <= arr.length <= 10^5
 * 	-10^6 <= arr[i] <= 10^6
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-absolute-difference/
// discuss: https://leetcode.com/problems/minimum-absolute-difference/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn minimum_abs_difference(arr: Vec<i32>) -> Vec<Vec<i32>> {
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1200() {
    }
}
