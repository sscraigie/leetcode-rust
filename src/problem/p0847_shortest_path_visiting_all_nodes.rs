/**
 * [847] Shortest Path Visiting All Nodes
 *
 * You have an undirected, connected graph of n nodes labeled from 0 to n - 1. You are given an array graph where graph[i] is a list of all the nodes connected with node i by an edge.
 * Return the length of the shortest path that visits every node. You may start and stop at any node, you may revisit nodes multiple times, and you may reuse edges.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/05/12/shortest1-graph.jpg" style="width: 222px; height: 183px;" />
 * Input: graph = [[1,2,3],[0],[0],[0]]
 * Output: 4
 * Explanation: One possible path is [1,0,2,0,3]
 * 
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/05/12/shortest2-graph.jpg" style="width: 382px; height: 222px;" />
 * Input: graph = [[1],[0,2,4],[1,3,4],[2],[1,2]]
 * Output: 4
 * Explanation: One possible path is [0,1,4,2,3]
 * 
 *  
 * Constraints:
 * 
 * 	n == graph.length
 * 	1 <= n <= 12
 * 	0 <= graph[i].length < n
 * 	graph[i] does not contain i.
 * 	If graph[a] contains b, then graph[b] contains a.
 * 	The input graph is always connected.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/shortest-path-visiting-all-nodes/
// discuss: https://leetcode.com/problems/shortest-path-visiting-all-nodes/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn shortest_path_length(graph: Vec<Vec<i32>>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_847() {
    }
}
