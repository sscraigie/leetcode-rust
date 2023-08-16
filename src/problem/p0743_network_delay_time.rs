/**
 * [743] Network Delay Time
 *
 * You are given a network of n nodes, labeled from 1 to n. You are also given times, a list of travel times as directed edges times[i] = (ui, vi, wi), where ui is the source node, vi is the target node, and wi is the time it takes for a signal to travel from source to target.
 * We will send a signal from a given node k. Return the minimum time it takes for all the n nodes to receive the signal. If it is impossible for all the n nodes to receive the signal, return -1.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/05/23/931_example_1.png" style="width: 217px; height: 239px;" />
 * Input: times = [[2,1,1],[2,3,1],[3,4,1]], n = 4, k = 2
 * Output: 2
 * 
 * <strong class="example">Example 2:
 * 
 * Input: times = [[1,2,1]], n = 2, k = 1
 * Output: 1
 * 
 * <strong class="example">Example 3:
 * 
 * Input: times = [[1,2,1]], n = 2, k = 2
 * Output: -1
 * 
 *  
 * Constraints:
 * 
 * 	1 <= k <= n <= 100
 * 	1 <= times.length <= 6000
 * 	times[i].length == 3
 * 	1 <= ui, vi <= n
 * 	ui != vi
 * 	0 <= wi <= 100
 * 	All the pairs (ui, vi) are unique. (i.e., no multiple edges.)
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/network-delay-time/
// discuss: https://leetcode.com/problems/network-delay-time/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_743() {
    }
}
