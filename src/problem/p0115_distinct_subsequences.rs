/**
 * [115] Distinct Subsequences
 *
 * Given two strings s and t, return the number of distinct <span data-keyword="subsequence-string">subsequences</span> of s which equals t.
 * The test cases are generated so that the answer fits on a 32-bit signed integer.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: s = "rabbbit", t = "rabbit"
 * Output: 3
 * Explanation:
 * As shown below, there are 3 ways you can generate "rabbit" from s.
 * <u>rabb</u>b<u>it</u>
 * <u>ra</u>b<u>bbit</u>
 * <u>rab</u>b<u>bit</u>
 * 
 * <strong class="example">Example 2:
 * 
 * Input: s = "babgbag", t = "bag"
 * Output: 5
 * Explanation:
 * As shown below, there are 5 ways you can generate "bag" from s.
 * <u>ba</u>b<u>g</u>bag
 * <u>ba</u>bgba<u>g</u>
 * <u>b</u>abgb<u>ag</u>
 * ba<u>b</u>gb<u>ag</u>
 * babg<u>bag</u>
 *  
 * Constraints:
 * 
 * 	1 <= s.length, t.length <= 1000
 * 	s and t consist of English letters.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/distinct-subsequences/
// discuss: https://leetcode.com/problems/distinct-subsequences/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_115() {
    }
}
