/**
 * [1984] Minimum Difference Between Highest and Lowest of K Scores
 *
 * You are given a 0-indexed integer array nums, where nums[i] represents the score of the i^th student. You are also given an integer k.
 * Pick the scores of any k students from the array so that the difference between the highest and the lowest of the k scores is minimized.
 * Return the minimum possible difference.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: nums = [90], k = 1
 * Output: 0
 * Explanation: There is one way to pick score(s) of one student:
 * - [<u>90</u>]. The difference between the highest and lowest score is 90 - 90 = 0.
 * The minimum possible difference is 0.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: nums = [9,4,1,7], k = 2
 * Output: 2
 * Explanation: There are six ways to pick score(s) of two students:
 * - [<u>9</u>,<u>4</u>,1,7]. The difference between the highest and lowest score is 9 - 4 = 5.
 * - [<u>9</u>,4,<u>1</u>,7]. The difference between the highest and lowest score is 9 - 1 = 8.
 * - [<u>9</u>,4,1,<u>7</u>]. The difference between the highest and lowest score is 9 - 7 = 2.
 * - [9,<u>4</u>,<u>1</u>,7]. The difference between the highest and lowest score is 4 - 1 = 3.
 * - [9,<u>4</u>,1,<u>7</u>]. The difference between the highest and lowest score is 7 - 4 = 3.
 * - [9,4,<u>1</u>,<u>7</u>]. The difference between the highest and lowest score is 7 - 1 = 6.
 * The minimum possible difference is 2.
 *  
 * Constraints:
 * 
 * 	1 <= k <= nums.length <= 1000
 * 	0 <= nums[i] <= 10^5
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-difference-between-highest-and-lowest-of-k-scores/
// discuss: https://leetcode.com/problems/minimum-difference-between-highest-and-lowest-of-k-scores/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn minimum_difference(nums: Vec<i32>, k: i32) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1984() {
    }
}
