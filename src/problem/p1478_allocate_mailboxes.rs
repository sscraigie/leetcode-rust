/**
 * [1478] Allocate Mailboxes
 *
 * Given the array houses where houses[i] is the location of the i^th house along a street and an integer k, allocate k mailboxes in the street.
 * Return the minimum total distance between each house and its nearest mailbox.
 * The test cases are generated so that the answer fits in a 32-bit integer.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/05/07/sample_11_1816.png" style="width: 454px; height: 154px;" />
 * Input: houses = [1,4,8,10,20], k = 3
 * Output: 5
 * Explanation: Allocate mailboxes in position 3, 9 and 20.
 * Minimum total distance from each houses to nearest mailboxes is |3-1| + |4-3| + |9-8| + |10-9| + |20-20| = 5 
 * 
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/05/07/sample_2_1816.png" style="width: 433px; height: 154px;" />
 * Input: houses = [2,3,5,12,18], k = 2
 * Output: 9
 * Explanation: Allocate mailboxes in position 3 and 14.
 * Minimum total distance from each houses to nearest mailboxes is |2-3| + |3-3| + |5-3| + |12-14| + |18-14| = 9.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= k <= houses.length <= 100
 * 	1 <= houses[i] <= 10^4
 * 	All the integers of houses are unique.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/allocate-mailboxes/
// discuss: https://leetcode.com/problems/allocate-mailboxes/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_distance(houses: Vec<i32>, k: i32) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1478() {
    }
}
