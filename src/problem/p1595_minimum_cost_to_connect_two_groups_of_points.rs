/**
 * [1595] Minimum Cost to Connect Two Groups of Points
 *
 * You are given two groups of points where the first group has size1 points, the second group has size2 points, and size1 >= size2.
 * The cost of the connection between any two points are given in an size1 x size2 matrix where cost[i][j] is the cost of connecting point i of the first group and point j of the second group. The groups are connected if each point in both groups is connected to one or more points in the opposite group. In other words, each point in the first group must be connected to at least one point in the second group, and each point in the second group must be connected to at least one point in the first group.
 * Return the minimum cost it takes to connect the two groups.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/03/ex1.jpg" style="width: 322px; height: 243px;" />
 * Input: cost = [[15, 96], [36, 2]]
 * Output: 17
 * Explanation: The optimal way of connecting the groups is:
 * 1--A
 * 2--B
 * This results in a total cost of 17.
 * 
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/03/ex2.jpg" style="width: 322px; height: 403px;" />
 * Input: cost = [[1, 3, 5], [4, 1, 1], [1, 5, 3]]
 * Output: 4
 * Explanation: The optimal way of connecting the groups is:
 * 1--A
 * 2--B
 * 2--C
 * 3--A
 * This results in a total cost of 4.
 * Note that there are multiple points connected to point 2 in the first group and point A in the second group. This does not matter as there is no limit to the number of points that can be connected. We only care about the minimum total cost.
 * 
 * <strong class="example">Example 3:
 * 
 * Input: cost = [[2, 5, 1], [3, 4, 7], [8, 1, 2], [6, 2, 4], [3, 8, 8]]
 * Output: 10
 * 
 *  
 * Constraints:
 * 
 * 	size1 == cost.length
 * 	size2 == cost[i].length
 * 	1 <= size1, size2 <= 12
 * 	size1 >= size2
 * 	0 <= cost[i][j] <= 100
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-cost-to-connect-two-groups-of-points/
// discuss: https://leetcode.com/problems/minimum-cost-to-connect-two-groups-of-points/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn connect_two_groups(cost: Vec<Vec<i32>>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1595() {
    }
}
