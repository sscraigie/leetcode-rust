/**
 * [830] Positions of Large Groups
 *
 * In a string <font face="monospace">s</font> of lowercase letters, these letters form consecutive groups of the same character.
 * For example, a string like s = "abbxxxxzyy" has the groups "a", "bb", "xxxx", "z", and "yy".
 * A group is identified by an interval [start, end], where start and end denote the start and end indices (inclusive) of the group. In the above example, "xxxx" has the interval [3,6].
 * A group is considered large if it has 3 or more characters.
 * Return the intervals of every large group sorted in increasing order by start index.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: s = "abbxxxxzzy"
 * Output: [[3,6]]
 * Explanation: "xxxx" is the only large group with start index 3 and end index 6.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: s = "abc"
 * Output: []
 * Explanation: We have groups "a", "b", and "c", none of which are large groups.
 * 
 * <strong class="example">Example 3:
 * 
 * Input: s = "abcdddeeeeaabbbcd"
 * Output: [[3,5],[6,9],[12,14]]
 * Explanation: The large groups are "ddd", "eeee", and "bbb".
 * 
 *  
 * Constraints:
 * 
 * 	1 <= s.length <= 1000
 * 	s contains lowercase English letters only.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/positions-of-large-groups/
// discuss: https://leetcode.com/problems/positions-of-large-groups/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn large_group_positions(s: String) -> Vec<Vec<i32>> {
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_830() {
    }
}
