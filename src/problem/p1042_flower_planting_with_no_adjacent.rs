/**
 * [1042] Flower Planting With No Adjacent
 *
 * You have n gardens, labeled from 1 to n, and an array paths where paths[i] = [xi, yi] describes a bidirectional path between garden xi to garden yi. In each garden, you want to plant one of 4 types of flowers.
 * All gardens have at most 3 paths coming into or leaving it.
 * Your task is to choose a flower type for each garden such that, for any two gardens connected by a path, they have different types of flowers.
 * Return any such a choice as an array answer, where answer[i] is the type of flower planted in the (i+1)^th garden. The flower types are denoted 1, 2, 3, or 4. It is guaranteed an answer exists.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: n = 3, paths = [[1,2],[2,3],[3,1]]
 * Output: [1,2,3]
 * Explanation:
 * Gardens 1 and 2 have different types.
 * Gardens 2 and 3 have different types.
 * Gardens 3 and 1 have different types.
 * Hence, [1,2,3] is a valid answer. Other valid answers include [1,2,4], [1,4,2], and [3,2,1].
 * 
 * <strong class="example">Example 2:
 * 
 * Input: n = 4, paths = [[1,2],[3,4]]
 * Output: [1,2,1,2]
 * 
 * <strong class="example">Example 3:
 * 
 * Input: n = 4, paths = [[1,2],[2,3],[3,4],[4,1],[1,3],[2,4]]
 * Output: [1,2,3,4]
 * 
 *  
 * Constraints:
 * 
 * 	1 <= n <= 10^4
 * 	0 <= paths.length <= 2 * 10^4
 * 	paths[i].length == 2
 * 	1 <= xi, yi <= n
 * 	xi != yi
 * 	Every garden has at most 3 paths coming into or leaving it.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/flower-planting-with-no-adjacent/
// discuss: https://leetcode.com/problems/flower-planting-with-no-adjacent/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn garden_no_adj(n: i32, paths: Vec<Vec<i32>>) -> Vec<i32> {
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1042() {
    }
}
