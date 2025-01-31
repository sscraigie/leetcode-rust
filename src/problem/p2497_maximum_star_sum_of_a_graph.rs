/**
 * [2497] Maximum Star Sum of a Graph
 *
 * There is an undirected graph consisting of n nodes numbered from 0 to n - 1. You are given a 0-indexed integer array vals of length n where vals[i] denotes the value of the i^th node.
 * You are also given a 2D integer array edges where edges[i] = [ai, bi] denotes that there exists an undirected edge connecting nodes ai and bi.
 * A star graph is a subgraph of the given graph having a center node containing 0 or more neighbors. In other words, it is a subset of edges of the given graph such that there exists a common node for all edges.
 * The image below shows star graphs with 3 and 4 neighbors respectively, centered at the blue node.
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/11/07/max-star-sum-descdrawio.png" style="width: 400px; height: 179px;" />
 * The star sum is the sum of the values of all the nodes present in the star graph.
 * Given an integer k, return the maximum star sum of a star graph containing at most k edges.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/11/07/max-star-sum-example1drawio.png" style="width: 300px; height: 291px;" />
 * Input: vals = [1,2,3,4,10,-10,-20], edges = [[0,1],[1,2],[1,3],[3,4],[3,5],[3,6]], k = 2
 * Output: 16
 * Explanation: The above diagram represents the input graph.
 * The star graph with the maximum star sum is denoted by blue. It is centered at 3 and includes its neighbors 1 and 4.
 * It can be shown it is not possible to get a star graph with a sum greater than 16.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: vals = [-5], edges = [], k = 0
 * Output: -5
 * Explanation: There is only one possible star graph, which is node 0 itself.
 * Hence, we return -5.
 * 
 *  
 * Constraints:
 * 
 * 	n == vals.length
 * 	1 <= n <= 10^5
 * 	-10^4 <= vals[i] <= 10^4
 * 	0 <= edges.length <= min(n * (n - 1) / 2, 10^5)
 * 	edges[i].length == 2
 * 	0 <= ai, bi <= n - 1
 * 	ai != bi
 * 	0 <= k <= n - 1
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-star-sum-of-a-graph/
// discuss: https://leetcode.com/problems/maximum-star-sum-of-a-graph/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_star_sum(vals: Vec<i32>, edges: Vec<Vec<i32>>, k: i32) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2497() {
    }
}
