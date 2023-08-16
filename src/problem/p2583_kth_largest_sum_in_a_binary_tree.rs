/**
 * [2583] Kth Largest Sum in a Binary Tree
 *
 * You are given the root of a binary tree and a positive integer k.
 * The level sum in the tree is the sum of the values of the nodes that are on the same level.
 * Return the k^th largest level sum in the tree (not necessarily distinct). If there are fewer than k levels in the tree, return -1.
 * Note that two nodes are on the same level if they have the same distance from the root.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/12/14/binaryytreeedrawio-2.png" style="width: 301px; height: 284px;" />
 * Input: root = [5,8,9,2,1,3,7,4,6], k = 2
 * Output: 13
 * Explanation: The level sums are the following:
 * - Level 1: 5.
 * - Level 2: 8 + 9 = 17.
 * - Level 3: 2 + 1 + 3 + 7 = 13.
 * - Level 4: 4 + 6 = 10.
 * The 2^nd largest level sum is 13.
 * 
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/12/14/treedrawio-3.png" style="width: 181px; height: 181px;" />
 * Input: root = [1,2,null,3], k = 1
 * Output: 3
 * Explanation: The largest level sum is 3.
 * 
 *  
 * Constraints:
 * 
 * 	The number of nodes in the tree is n.
 * 	2 <= n <= 10^5
 * 	1 <= Node.val <= 10^6
 * 	1 <= k <= n
 * 
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/kth-largest-sum-in-a-binary-tree/
// discuss: https://leetcode.com/problems/kth-largest-sum-in-a-binary-tree/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn kth_largest_level_sum(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i64 {
        
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2583() {
    }
}
