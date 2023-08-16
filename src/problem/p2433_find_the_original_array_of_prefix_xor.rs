/**
 * [2433] Find The Original Array of Prefix Xor
 *
 * You are given an integer array pref of size n. Find and return the array arr of size n that satisfies:
 * 
 * 	pref[i] = arr[0] ^ arr[1] ^ ... ^ arr[i].
 * 
 * Note that ^ denotes the bitwise-xor operation.
 * It can be proven that the answer is unique.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: pref = [5,2,0,3,1]
 * Output: [5,7,2,3,2]
 * Explanation: From the array [5,7,2,3,2] we have the following:
 * - pref[0] = 5.
 * - pref[1] = 5 ^ 7 = 2.
 * - pref[2] = 5 ^ 7 ^ 2 = 0.
 * - pref[3] = 5 ^ 7 ^ 2 ^ 3 = 3.
 * - pref[4] = 5 ^ 7 ^ 2 ^ 3 ^ 2 = 1.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: pref = [13]
 * Output: [13]
 * Explanation: We have pref[0] = arr[0] = 13.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= pref.length <= 10^5
 * 	0 <= pref[i] <= 10^6
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-the-original-array-of-prefix-xor/
// discuss: https://leetcode.com/problems/find-the-original-array-of-prefix-xor/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_array(pref: Vec<i32>) -> Vec<i32> {
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2433() {
    }
}
