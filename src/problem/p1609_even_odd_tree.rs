/**
 * [1609] Even Odd Tree
 *
 * A binary tree is named Even-Odd if it meets the following conditions:
 * 
 * 	The root of the binary tree is at level index 0, its children are at level index 1, their children are at level index 2, etc.
 * 	For every even-indexed level, all nodes at the level have odd integer values in strictly increasing order (from left to right).
 * 	For every odd-indexed level, all nodes at the level have even integer values in strictly decreasing order (from left to right).
 * 
 * Given the root of a binary tree, return true if the binary tree is Even-Odd, otherwise return false.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/15/sample_1_1966.png" style="width: 362px; height: 229px;" />
 * Input: root = [1,10,4,3,null,7,9,12,8,6,null,null,2]
 * Output: true
 * Explanation: The node values on each level are:
 * Level 0: [1]
 * Level 1: [10,4]
 * Level 2: [3,7,9]
 * Level 3: [12,8,6,2]
 * Since levels 0 and 2 are all odd and increasing and levels 1 and 3 are all even and decreasing, the tree is Even-Odd.
 * 
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/15/sample_2_1966.png" style="width: 363px; height: 167px;" />
 * Input: root = [5,4,2,3,3,7]
 * Output: false
 * Explanation: The node values on each level are:
 * Level 0: [5]
 * Level 1: [4,2]
 * Level 2: [3,3,7]
 * Node values in level 2 must be in strictly increasing order, so the tree is not Even-Odd.
 * 
 * <strong class="example">Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/09/22/sample_1_333_1966.png" style="width: 363px; height: 167px;" />
 * Input: root = [5,9,1,3,5,7]
 * Output: false
 * Explanation: Node values in the level 1 should be even integers.
 * 
 *  
 * Constraints:
 * 
 * 	The number of nodes in the tree is in the range [1, 10^5].
 * 	1 <= Node.val <= 10^6
 * 
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/even-odd-tree/
// discuss: https://leetcode.com/problems/even-odd-tree/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn is_even_odd_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1609() {
    }
}
