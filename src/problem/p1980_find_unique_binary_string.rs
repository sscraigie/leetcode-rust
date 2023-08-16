/**
 * [1980] Find Unique Binary String
 *
 * Given an array of strings nums containing n unique binary strings each of length n, return a binary string of length n that does not appear in nums. If there are multiple answers, you may return any of them.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: nums = ["01","10"]
 * Output: "11"
 * Explanation: "11" does not appear in nums. "00" would also be correct.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: nums = ["00","01"]
 * Output: "11"
 * Explanation: "11" does not appear in nums. "10" would also be correct.
 * 
 * <strong class="example">Example 3:
 * 
 * Input: nums = ["111","011","001"]
 * Output: "101"
 * Explanation: "101" does not appear in nums. "000", "010", "100", and "110" would also be correct.
 * 
 *  
 * Constraints:
 * 
 * 	n == nums.length
 * 	1 <= n <= 16
 * 	nums[i].length == n
 * 	nums[i] is either '0' or '1'.
 * 	All the strings of nums are unique.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-unique-binary-string/
// discuss: https://leetcode.com/problems/find-unique-binary-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        String::new()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1980() {
    }
}
