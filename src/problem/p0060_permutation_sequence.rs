/**
 * [60] Permutation Sequence
 *
 * The set [1, 2, 3, ..., n] contains a total of n! unique permutations.
 * By listing and labeling all of the permutations in order, we get the following sequence for n = 3:
 * <ol>
 * 	"123"
 * 	"132"
 * 	"213"
 * 	"231"
 * 	"312"
 * 	"321"
 * </ol>
 * Given n and k, return the k^th permutation sequence.
 *  
 * <strong class="example">Example 1:
 * Input: n = 3, k = 3
 * Output: "213"
 * <strong class="example">Example 2:
 * Input: n = 4, k = 9
 * Output: "2314"
 * <strong class="example">Example 3:
 * Input: n = 3, k = 1
 * Output: "123"
 *  
 * Constraints:
 * 
 * 	1 <= n <= 9
 * 	1 <= k <= n!
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/permutation-sequence/
// discuss: https://leetcode.com/problems/permutation-sequence/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        String::new()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_60() {
    }
}
