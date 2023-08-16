/**
 * [2673] Make Costs of Paths Equal in a Binary Tree
 *
 * You are given an integer n representing the number of nodes in a perfect binary tree consisting of nodes numbered from 1 to n. The root of the tree is node 1 and each node i in the tree has two children where the left child is the node 2 * i and the right child is 2 * i + 1.
 * Each node in the tree also has a cost represented by a given 0-indexed integer array cost of size n where cost[i] is the cost of node i + 1. You are allowed to increment the cost of any node by 1 any number of times.
 * Return the minimum number of increments you need to make the cost of paths from the root to each leaf node equal.
 * Note:
 * 
 * 	A perfect binary tree is a tree where each node, except the leaf nodes, has exactly 2 children.
 * 	The cost of a path is the sum of costs of nodes in the path.
 * 
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2023/04/04/binaryytreeedrawio-4.png" />
 * Input: n = 7, cost = [1,5,2,2,3,3,1]
 * Output: 6
 * Explanation: We can do the following increments:
 * - Increase the cost of node 4 one time.
 * - Increase the cost of node 3 three times.
 * - Increase the cost of node 7 two times.
 * Each path from the root to a leaf will have a total cost of 9.
 * The total increments we did is 1 + 3 + 2 = 6.
 * It can be shown that this is the minimum answer we can achieve.
 * 
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2023/04/04/binaryytreee2drawio.png" style="width: 205px; height: 151px;" />
 * Input: n = 3, cost = [5,3,3]
 * Output: 0
 * Explanation: The two paths already have equal total costs, so no increments are needed.
 * 
 *  
 * Constraints:
 * 
 * 	3 <= n <= 10^5
 * 	n + 1 is a power of 2
 * 	cost.length == n
 * 	1 <= cost[i] <= 10^4
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/make-costs-of-paths-equal-in-a-binary-tree/
// discuss: https://leetcode.com/problems/make-costs-of-paths-equal-in-a-binary-tree/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_increments(n: i32, cost: Vec<i32>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2673() {
    }
}
