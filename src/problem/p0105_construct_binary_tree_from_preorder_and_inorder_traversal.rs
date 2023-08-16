/**
 * [105] Construct Binary Tree from Preorder and Inorder Traversal
 *
 * Given two integer arrays preorder and inorder where preorder is the preorder traversal of a binary tree and inorder is the inorder traversal of the same tree, construct and return the binary tree.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/02/19/tree.jpg" style="width: 277px; height: 302px;" />
 * Input: preorder = [3,9,20,15,7], inorder = [9,3,15,20,7]
 * Output: [3,9,20,null,null,15,7]
 * 
 * <strong class="example">Example 2:
 * 
 * Input: preorder = [-1], inorder = [-1]
 * Output: [-1]
 * 
 *  
 * Constraints:
 * 
 * 	1 <= preorder.length <= 3000
 * 	inorder.length == preorder.length
 * 	-3000 <= preorder[i], inorder[i] <= 3000
 * 	preorder and inorder consist of unique values.
 * 	Each value of inorder also appears in preorder.
 * 	preorder is guaranteed to be the preorder traversal of the tree.
 * 	inorder is guaranteed to be the inorder traversal of the tree.
 * 
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/
// discuss: https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode::new(0))))
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_105() {
    }
}
