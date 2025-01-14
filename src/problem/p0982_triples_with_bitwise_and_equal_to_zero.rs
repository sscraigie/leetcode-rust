/**
 * [982] Triples with Bitwise AND Equal To Zero
 *
 * Given an integer array nums, return the number of AND triples.
 * An AND triple is a triple of indices (i, j, k) such that:
 * 
 * 	0 <= i < nums.length
 * 	0 <= j < nums.length
 * 	0 <= k < nums.length
 * 	nums[i] &amp; nums[j] &amp; nums[k] == 0, where &amp; represents the bitwise-AND operator.
 * 
 *  
 * <strong class="example">Example 1:
 * 
 * Input: nums = [2,1,3]
 * Output: 12
 * Explanation: We could choose the following i, j, k triples:
 * (i=0, j=0, k=1) : 2 &amp; 2 &amp; 1
 * (i=0, j=1, k=0) : 2 &amp; 1 &amp; 2
 * (i=0, j=1, k=1) : 2 &amp; 1 &amp; 1
 * (i=0, j=1, k=2) : 2 &amp; 1 &amp; 3
 * (i=0, j=2, k=1) : 2 &amp; 3 &amp; 1
 * (i=1, j=0, k=0) : 1 &amp; 2 &amp; 2
 * (i=1, j=0, k=1) : 1 &amp; 2 &amp; 1
 * (i=1, j=0, k=2) : 1 &amp; 2 &amp; 3
 * (i=1, j=1, k=0) : 1 &amp; 1 &amp; 2
 * (i=1, j=2, k=0) : 1 &amp; 3 &amp; 2
 * (i=2, j=0, k=1) : 3 &amp; 2 &amp; 1
 * (i=2, j=1, k=0) : 3 &amp; 1 &amp; 2
 * 
 * <strong class="example">Example 2:
 * 
 * Input: nums = [0,0,0]
 * Output: 27
 * 
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 1000
 * 	0 <= nums[i] < 2^16
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/triples-with-bitwise-and-equal-to-zero/
// discuss: https://leetcode.com/problems/triples-with-bitwise-and-equal-to-zero/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_triplets(nums: Vec<i32>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_982() {
    }
}
