/**
 * [658] Find K Closest Elements
 *
 * Given a sorted integer array arr, two integers k and x, return the k closest integers to x in the array. The result should also be sorted in ascending order.
 * An integer a is closer to x than an integer b if:
 * 
 * 	|a - x| < |b - x|, or
 * 	|a - x| == |b - x| and a < b
 * 
 *  
 * <strong class="example">Example 1:
 * Input: arr = [1,2,3,4,5], k = 4, x = 3
 * Output: [1,2,3,4]
 * <strong class="example">Example 2:
 * Input: arr = [1,2,3,4,5], k = 4, x = -1
 * Output: [1,2,3,4]
 *  
 * Constraints:
 * 
 * 	1 <= k <= arr.length
 * 	1 <= arr.length <= 10^4
 * 	arr is sorted in ascending order.
 * 	-10^4 <= arr[i], x <= 10^4
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-k-closest-elements/
// discuss: https://leetcode.com/problems/find-k-closest-elements/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_658() {
    }
}
