/**
 * [954] Array of Doubled Pairs
 *
 * Given an integer array of even length arr, return true if it is possible to reorder arr such that arr[2 * i + 1] = 2 * arr[2 * i] for every 0 <= i < len(arr) / 2, or false otherwise.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: arr = [3,1,3,6]
 * Output: false
 * 
 * <strong class="example">Example 2:
 * 
 * Input: arr = [2,1,2,6]
 * Output: false
 * 
 * <strong class="example">Example 3:
 * 
 * Input: arr = [4,-2,2,-4]
 * Output: true
 * Explanation: We can take two groups, [-2,-4] and [2,4] to form [-2,-4,2,4] or [2,4,-2,-4].
 * 
 *  
 * Constraints:
 * 
 * 	2 <= arr.length <= 3 * 10^4
 * 	arr.length is even.
 * 	-10^5 <= arr[i] <= 10^5
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/array-of-doubled-pairs/
// discuss: https://leetcode.com/problems/array-of-doubled-pairs/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn can_reorder_doubled(arr: Vec<i32>) -> bool {
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_954() {
    }
}
