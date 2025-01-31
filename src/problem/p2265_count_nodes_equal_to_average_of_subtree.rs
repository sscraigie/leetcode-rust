/**
 * [2265] Count Nodes Equal to Average of Subtree
 *
 * Given the root of a binary tree, return the number of nodes where the value of the node is equal to the average of the values in its subtree.
 * Note:
 * 
 * 	The average of n elements is the sum of the n elements divided by n and rounded down to the nearest integer.
 * 	A subtree of root is a tree consisting of root and all of its descendants.
 * 
 *  
 * <strong class="example">Example 1:
 * <img src="https://assets.leetcode.com/uploads/2022/03/15/image-20220315203925-1.png" style="width: 300px; height: 212px;" />
 * Input: root = [4,8,5,0,1,null,6]
 * Output: 5
 * Explanation: 
 * For the node with value 4: The average of its subtree is (4 + 8 + 5 + 0 + 1 + 6) / 6 = 24 / 6 = 4.
 * For the node with value 5: The average of its subtree is (5 + 6) / 2 = 11 / 2 = 5.
 * For the node with value 0: The average of its subtree is 0 / 1 = 0.
 * For the node with value 1: The average of its subtree is 1 / 1 = 1.
 * For the node with value 6: The average of its subtree is 6 / 1 = 6.
 * 
 * <strong class="example">Example 2:
 * <img src="https://assets.leetcode.com/uploads/2022/03/26/image-20220326133920-1.png" style="width: 80px; height: 76px;" />
 * Input: root = [1]
 * Output: 1
 * Explanation: For the node with value 1: The average of its subtree is 1 / 1 = 1.
 * 
 *  
 * Constraints:
 * 
 * 	The number of nodes in the tree is in the range [1, 1000].
 * 	0 <= Node.val <= 1000
 * 
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/count-nodes-equal-to-average-of-subtree/
// discuss: https://leetcode.com/problems/count-nodes-equal-to-average-of-subtree/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn average_of_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2265() {
    }
}
