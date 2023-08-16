/**
 * [336] Palindrome Pairs
 *
 * You are given a 0-indexed array of unique strings words.
 * A palindrome pair is a pair of integers (i, j) such that:
 * 
 * 	0 <= i, j < words.length,
 * 	i != j, and
 * 	words[i] + words[j] (the concatenation of the two strings) is a <span data-keyword="palindrome-string">palindrome</span>.
 * 
 * Return an array of all the palindrome pairs of words.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: words = ["abcd","dcba","lls","s","sssll"]
 * Output: [[0,1],[1,0],[3,2],[2,4]]
 * Explanation: The palindromes are ["abcddcba","dcbaabcd","slls","llssssll"]
 * 
 * <strong class="example">Example 2:
 * 
 * Input: words = ["bat","tab","cat"]
 * Output: [[0,1],[1,0]]
 * Explanation: The palindromes are ["battab","tabbat"]
 * 
 * <strong class="example">Example 3:
 * 
 * Input: words = ["a",""]
 * Output: [[0,1],[1,0]]
 * Explanation: The palindromes are ["a","a"]
 * 
 *  
 * Constraints:
 * 
 * 	1 <= words.length <= 5000
 * 	0 <= words[i].length <= 300
 * 	words[i] consists of lowercase English letters.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/palindrome-pairs/
// discuss: https://leetcode.com/problems/palindrome-pairs/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_336() {
    }
}
