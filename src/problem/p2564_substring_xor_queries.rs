/**
 * [2564] Substring XOR Queries
 *
 * You are given a binary string s, and a 2D integer array queries where queries[i] = [firsti, secondi].
 * For the i^th query, find the shortest substring of s whose decimal value, val, yields secondi when bitwise XORed with firsti. In other words, val ^ firsti == secondi.
 * The answer to the i^th query is the endpoints (0-indexed) of the substring [lefti, righti] or [-1, -1] if no such substring exists. If there are multiple answers, choose the one with the minimum lefti.
 * Return an array ans where ans[i] = [lefti, righti] is the answer to the i^th query.
 * A substring is a contiguous non-empty sequence of characters within a string.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: s = "101101", queries = [[0,5],[1,2]]
 * Output: [[0,2],[2,3]]
 * Explanation: For the first query the substring in range [0,2] is "101" which has a decimal value of 5, and 5 ^ 0 = 5, hence the answer to the first query is [0,2]. In the second query, the substring in range [2,3] is "11", and has a decimal value of 3, and 3 ^ 1 = 2. So, [2,3] is returned for the second query. 
 * 
 * <strong class="example">Example 2:
 * 
 * Input: s = "0101", queries = [[12,8]]
 * Output: [[-1,-1]]
 * Explanation: In this example there is no substring that answers the query, hence [-1,-1] is returned.
 * 
 * <strong class="example">Example 3:
 * 
 * Input: s = "1", queries = [[4,5]]
 * Output: [[0,0]]
 * Explanation: For this example, the substring in range [0,0] has a decimal value of 1, and 1 ^ 4 = 5. So, the answer is [0,0].
 * 
 *  
 * Constraints:
 * 
 * 	1 <= s.length <= 10^4
 * 	s[i] is either '0' or '1'.
 * 	1 <= queries.length <= 10^5
 * 	0 <= firsti, secondi <= 10^9
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/substring-xor-queries/
// discuss: https://leetcode.com/problems/substring-xor-queries/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn substring_xor_queries(s: String, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2564() {
    }
}
