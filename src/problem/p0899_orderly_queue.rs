/**
 * [899] Orderly Queue
 *
 * You are given a string s and an integer k. You can choose one of the first k letters of s and append it at the end of the string..
 * Return the lexicographically smallest string you could have after applying the mentioned step any number of moves.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: s = "cba", k = 1
 * Output: "acb"
 * Explanation: 
 * In the first move, we move the 1^st character 'c' to the end, obtaining the string "bac".
 * In the second move, we move the 1^st character 'b' to the end, obtaining the final result "acb".
 * 
 * <strong class="example">Example 2:
 * 
 * Input: s = "baaca", k = 3
 * Output: "aaabc"
 * Explanation: 
 * In the first move, we move the 1^st character 'b' to the end, obtaining the string "aacab".
 * In the second move, we move the 3^rd character 'c' to the end, obtaining the final result "aaabc".
 * 
 *  
 * Constraints:
 * 
 * 	1 <= k <= s.length <= 1000
 * 	s consist of lowercase English letters.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/orderly-queue/
// discuss: https://leetcode.com/problems/orderly-queue/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn orderly_queue(s: String, k: i32) -> String {
        String::new()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_899() {
    }
}
