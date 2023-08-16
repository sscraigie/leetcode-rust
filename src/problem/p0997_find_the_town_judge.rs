/**
 * [997] Find the Town Judge
 *
 * In a town, there are n people labeled from 1 to n. There is a rumor that one of these people is secretly the town judge.
 * If the town judge exists, then:
 * <ol>
 * 	The town judge trusts nobody.
 * 	Everybody (except for the town judge) trusts the town judge.
 * 	There is exactly one person that satisfies properties 1 and 2.
 * </ol>
 * You are given an array trust where trust[i] = [ai, bi] representing that the person labeled ai trusts the person labeled bi. If a trust relationship does not exist in trust array, then such a trust relationship does not exist.
 * Return the label of the town judge if the town judge exists and can be identified, or return -1 otherwise.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: n = 2, trust = [[1,2]]
 * Output: 2
 * 
 * <strong class="example">Example 2:
 * 
 * Input: n = 3, trust = [[1,3],[2,3]]
 * Output: 3
 * 
 * <strong class="example">Example 3:
 * 
 * Input: n = 3, trust = [[1,3],[2,3],[3,1]]
 * Output: -1
 * 
 *  
 * Constraints:
 * 
 * 	1 <= n <= 1000
 * 	0 <= trust.length <= 10^4
 * 	trust[i].length == 2
 * 	All the pairs of trust are unique.
 * 	ai != bi
 * 	1 <= ai, bi <= n
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-the-town-judge/
// discuss: https://leetcode.com/problems/find-the-town-judge/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_997() {
    }
}
