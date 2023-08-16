/**
 * [839] Similar String Groups
 *
 * Two strings, X and Y, are considered similar if either they are identical or we can make them equivalent by swapping at most two letters (in distinct positions) within the string X.
 * For example, "tars" and "rats" are similar (swapping at positions 0 and 2), and "rats" and "arts" are similar, but "star" is not similar to "tars", "rats", or "arts".
 * Together, these form two connected groups by similarity: {"tars", "rats", "arts"} and {"star"}.  Notice that "tars" and "arts" are in the same group even though they are not similar.  Formally, each group is such that a word is in the group if and only if it is similar to at least one other word in the group.
 * We are given a list strs of strings where every string in strs is an anagram of every other string in strs. How many groups are there?
 *  
 * <strong class="example">Example 1:
 * 
 * Input: strs = ["tars","rats","arts","star"]
 * Output: 2
 * 
 * <strong class="example">Example 2:
 * 
 * Input: strs = ["omv","ovm"]
 * Output: 1
 * 
 *  
 * Constraints:
 * 
 * 	1 <= strs.length <= 300
 * 	1 <= strs[i].length <= 300
 * 	strs[i] consists of lowercase letters only.
 * 	All words in strs have the same length and are anagrams of each other.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/similar-string-groups/
// discuss: https://leetcode.com/problems/similar-string-groups/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn num_similar_groups(strs: Vec<String>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_839() {
    }
}
