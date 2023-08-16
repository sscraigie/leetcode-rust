/**
 * [1202] Smallest String With Swaps
 *
 * You are given a string s, and an array of pairs of indices in the string pairs where pairs[i] = [a, b] indicates 2 indices(0-indexed) of the string.
 * You can swap the characters at any pair of indices in the given pairs any number of times.
 * Return the lexicographically smallest string that s can be changed to after using the swaps.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: s = "dcab", pairs = [[0,3],[1,2]]
 * Output: "bacd"
 * Explaination: 
 * Swap s[0] and s[3], s = "bcad"
 * Swap s[1] and s[2], s = "bacd"
 * 
 * <strong class="example">Example 2:
 * 
 * Input: s = "dcab", pairs = [[0,3],[1,2],[0,2]]
 * Output: "abcd"
 * Explaination: 
 * Swap s[0] and s[3], s = "bcad"
 * Swap s[0] and s[2], s = "acbd"
 * Swap s[1] and s[2], s = "abcd"
 * <strong class="example">Example 3:
 * 
 * Input: s = "cba", pairs = [[0,1],[1,2]]
 * Output: "abc"
 * Explaination: 
 * Swap s[0] and s[1], s = "bca"
 * Swap s[1] and s[2], s = "bac"
 * Swap s[0] and s[1], s = "abc"
 * 
 *  
 * Constraints:
 * 
 * 	1 <= s.length <= 10^5
 * 	0 <= pairs.length <= 10^5
 * 	0 <= pairs[i][0], pairs[i][1] < s.length
 * 	s only contains lower case English letters.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/smallest-string-with-swaps/
// discuss: https://leetcode.com/problems/smallest-string-with-swaps/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn smallest_string_with_swaps(s: String, pairs: Vec<Vec<i32>>) -> String {
        String::new()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1202() {
    }
}
