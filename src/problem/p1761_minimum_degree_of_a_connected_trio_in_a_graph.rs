/**
 * [1761] Minimum Degree of a Connected Trio in a Graph
 *
 * You are given an undirected graph. You are given an integer n which is the number of nodes in the graph and an array edges, where each edges[i] = [ui, vi] indicates that there is an undirected edge between ui and vi.
 * A connected trio is a set of three nodes where there is an edge between every pair of them.
 * The degree of a connected trio is the number of edges where one endpoint is in the trio, and the other is not.
 * Return the minimum degree of a connected trio in the graph, or -1 if the graph has no connected trios.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/26/trios1.png" style="width: 388px; height: 164px;" />
 * Input: n = 6, edges = [[1,2],[1,3],[3,2],[4,1],[5,2],[3,6]]
 * Output: 3
 * Explanation: There is exactly one trio, which is [1,2,3]. The edges that form its degree are bolded in the figure above.
 * 
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/01/26/trios2.png" style="width: 388px; height: 164px;" />
 * Input: n = 7, edges = [[1,3],[4,1],[4,3],[2,5],[5,6],[6,7],[7,5],[2,6]]
 * Output: 0
 * Explanation: There are exactly three trios:
 * 1) [1,4,3] with degree 0.
 * 2) [2,5,6] with degree 2.
 * 3) [5,6,7] with degree 2.
 * 
 *  
 * Constraints:
 * 
 * 	2 <= n <= 400
 * 	edges[i].length == 2
 * 	1 <= edges.length <= n * (n-1) / 2
 * 	1 <= ui, vi <= n
 * 	ui != vi
 * 	There are no repeated edges.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-degree-of-a-connected-trio-in-a-graph/
// discuss: https://leetcode.com/problems/minimum-degree-of-a-connected-trio-in-a-graph/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_trio_degree(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1761() {
    }
}
