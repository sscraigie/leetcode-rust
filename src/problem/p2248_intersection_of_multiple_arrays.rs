/**
 * [2248] Intersection of Multiple Arrays
 *
 * Given a 2D integer array nums where nums[i] is a non-empty array of distinct positive integers, return the list of integers that are present in each array of nums sorted in ascending order.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: nums = [[<u>3</u>,1,2,<u>4</u>,5],[1,2,<u>3</u>,<u>4</u>],[<u>3</u>,<u>4</u>,5,6]]
 * Output: [3,4]
 * Explanation: 
 * The only integers present in each of nums[0] = [<u>3</u>,1,2,<u>4</u>,5], nums[1] = [1,2,<u>3</u>,<u>4</u>], and nums[2] = [<u>3</u>,<u>4</u>,5,6] are 3 and 4, so we return [3,4].
 * <strong class="example">Example 2:
 * 
 * Input: nums = [[1,2,3],[4,5,6]]
 * Output: []
 * Explanation: 
 * There does not exist any integer present both in nums[0] and nums[1], so we return an empty list [].
 * 
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 1000
 * 	1 <= sum(nums[i].length) <= 1000
 * 	1 <= nums[i][j] <= 1000
 * 	All the values of nums[i] are unique.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/intersection-of-multiple-arrays/
// discuss: https://leetcode.com/problems/intersection-of-multiple-arrays/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn intersection(nums: Vec<Vec<i32>>) -> Vec<i32> {
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2248() {
    }
}
