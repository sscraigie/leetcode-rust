/**
 * [886] Possible Bipartition
 *
 * We want to split a group of n people (labeled from 1 to n) into two groups of any size. Each person may dislike some other people, and they should not go into the same group.
 * Given the integer n and the array dislikes where dislikes[i] = [ai, bi] indicates that the person labeled ai does not like the person labeled bi, return true if it is possible to split everyone into two groups in this way.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: n = 4, dislikes = [[1,2],[1,3],[2,4]]
 * Output: true
 * Explanation: The first group has [1,4], and the second group has [2,3].
 * 
 * <strong class="example">Example 2:
 * 
 * Input: n = 3, dislikes = [[1,2],[1,3],[2,3]]
 * Output: false
 * Explanation: We need at least 3 groups to divide them. We cannot put them in two groups.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= n <= 2000
 * 	0 <= dislikes.length <= 10^4
 * 	dislikes[i].length == 2
 * 	1 <= ai < bi <= n
 * 	All the pairs of dislikes are unique.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/possible-bipartition/
// discuss: https://leetcode.com/problems/possible-bipartition/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_886() {
    }
}
