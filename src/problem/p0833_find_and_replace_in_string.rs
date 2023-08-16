/**
 * [833] Find And Replace in String
 *
 * You are given a 0-indexed string s that you must perform k replacement operations on. The replacement operations are given as three 0-indexed parallel arrays, indices, sources, and targets, all of length k.
 * To complete the i^th replacement operation:
 * <ol>
 * 	Check if the substring sources[i] occurs at index indices[i] in the original string s.
 * 	If it does not occur, do nothing.
 * 	Otherwise if it does occur, replace that substring with targets[i].
 * </ol>
 * For example, if s = "<u>ab</u>cd", indices[i] = 0, sources[i] = "ab", and targets[i] = "eee", then the result of this replacement will be "<u>eee</u>cd".
 * All replacement operations must occur simultaneously, meaning the replacement operations should not affect the indexing of each other. The testcases will be generated such that the replacements will not overlap.
 * 
 * 	For example, a testcase with s = "abc", indices = [0, 1], and sources = ["ab","bc"] will not be generated because the "ab" and "bc" replacements overlap.
 * 
 * Return the resulting string after performing all replacement operations on s.
 * A substring is a contiguous sequence of characters in a string.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/06/12/833-ex1.png" style="width: 411px; height: 251px;" />
 * Input: s = "abcd", indices = [0, 2], sources = ["a", "cd"], targets = ["eee", "ffff"]
 * Output: "eeebffff"
 * Explanation:
 * "a" occurs at index 0 in s, so we replace it with "eee".
 * "cd" occurs at index 2 in s, so we replace it with "ffff".
 * 
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/06/12/833-ex2-1.png" style="width: 411px; height: 251px;" />
 * Input: s = "abcd", indices = [0, 2], sources = ["ab","ec"], targets = ["eee","ffff"]
 * Output: "eeecd"
 * Explanation:
 * "ab" occurs at index 0 in s, so we replace it with "eee".
 * "ec" does not occur at index 2 in s, so we do nothing.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= s.length <= 1000
 * 	k == indices.length == sources.length == targets.length
 * 	1 <= k <= 100
 * 	0 <= indexes[i] < s.length
 * 	1 <= sources[i].length, targets[i].length <= 50
 * 	s consists of only lowercase English letters.
 * 	sources[i] and targets[i] consist of only lowercase English letters.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-and-replace-in-string/
// discuss: https://leetcode.com/problems/find-and-replace-in-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_replace_string(s: String, indices: Vec<i32>, sources: Vec<String>, targets: Vec<String>) -> String {
        String::new()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_833() {
    }
}
