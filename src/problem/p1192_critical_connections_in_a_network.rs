/**
 * [1192] Critical Connections in a Network
 *
 * There are n servers numbered from 0 to n - 1 connected by undirected server-to-server connections forming a network where connections[i] = [ai, bi] represents a connection between servers ai and bi. Any server can reach other servers directly or indirectly through the network.
 * A critical connection is a connection that, if removed, will make some servers unable to reach some other server.
 * Return all critical connections in the network in any order.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/09/03/1537_ex1_2.png" style="width: 198px; height: 248px;" />
 * Input: n = 4, connections = [[0,1],[1,2],[2,0],[1,3]]
 * Output: [[1,3]]
 * Explanation: [[3,1]] is also accepted.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: n = 2, connections = [[0,1]]
 * Output: [[0,1]]
 * 
 *  
 * Constraints:
 * 
 * 	2 <= n <= 10^5
 * 	n - 1 <= connections.length <= 10^5
 * 	0 <= ai, bi <= n - 1
 * 	ai != bi
 * 	There are no repeated connections.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/critical-connections-in-a-network/
// discuss: https://leetcode.com/problems/critical-connections-in-a-network/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn critical_connections(n: i32, connections: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1192() {
    }
}
