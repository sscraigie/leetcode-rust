/**
 * [1643] Kth Smallest Instructions
 *
 * Bob is standing at cell (0, 0), and he wants to reach destination: (row, column). He can only travel right and down. You are going to help Bob by providing instructions for him to reach destination.
 * The instructions are represented as a string, where each character is either:
 * 
 * 	'H', meaning move horizontally (go right), or
 * 	'V', meaning move vertically (go down).
 * 
 * Multiple instructions will lead Bob to destination. For example, if destination is (2, 3), both "HHHVV" and "HVHVH" are valid instructions.
 * However, Bob is very picky. Bob has a lucky number k, and he wants the k^th lexicographically smallest instructions that will lead him to destination. k is 1-indexed.
 * Given an integer array destination and an integer k, return the k^th lexicographically smallest instructions that will take Bob to destination.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/12/ex1.png" style="width: 300px; height: 229px;" />
 * 
 * Input: destination = [2,3], k = 1
 * Output: "HHHVV"
 * Explanation: All the instructions that reach (2, 3) in lexicographic order are as follows:
 * ["HHHVV", "HHVHV", "HHVVH", "HVHHV", "HVHVH", "HVVHH", "VHHHV", "VHHVH", "VHVHH", "VVHHH"].
 * 
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/12/ex2.png" style="width: 300px; height: 229px;" />
 * 
 * Input: destination = [2,3], k = 2
 * Output: "HHVHV"
 * 
 * <strong class="example">Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/12/ex3.png" style="width: 300px; height: 229px;" />
 * 
 * Input: destination = [2,3], k = 3
 * Output: "HHVVH"
 * 
 *  
 * Constraints:
 * 
 * 	destination.length == 2
 * 	1 <= row, column <= 15
 * 	1 <= k <= nCr(row + column, row), where nCr(a, b) denotes a choose b​​​​​.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/kth-smallest-instructions/
// discuss: https://leetcode.com/problems/kth-smallest-instructions/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn kth_smallest_path(destination: Vec<i32>, k: i32) -> String {
        String::new()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1643() {
    }
}
