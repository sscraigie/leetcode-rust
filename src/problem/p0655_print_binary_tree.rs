/**
 * [655] Print Binary Tree
 *
 * Given the root of a binary tree, construct a 0-indexed m x n string matrix res that represents a formatted layout of the tree. The formatted layout matrix should be constructed using the following rules:
 * 
 * 	The height of the tree is height and the number of rows m should be equal to height + 1.
 * 	The number of columns n should be equal to 2^height+1 - 1.
 * 	Place the root node in the middle of the top row (more formally, at location res[0][(n-1)/2]).
 * 	For each node that has been placed in the matrix at position res[r][c], place its left child at res[r+1][c-2^height-r-1] and its right child at res[r+1][c+2^height-r-1].
 * 	Continue this process until all the nodes in the tree have been placed.
 * 	Any empty cells should contain the empty string "".
 * 
 * Return the constructed matrix res.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/05/03/print1-tree.jpg" style="width: 141px; height: 181px;" />
 * Input: root = [1,2]
 * Output: 
 * [["","1",""],
 *  ["2","",""]]
 * 
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/05/03/print2-tree.jpg" style="width: 207px; height: 302px;" />
 * Input: root = [1,2,3,null,4]
 * Output: 
 * [["","","","1","","",""],
 *  ["","2","","","","3",""],
 *  ["","","4","","","",""]]
 * 
 *  
 * Constraints:
 * 
 * 	The number of nodes in the tree is in the range [1, 2^10].
 * 	-99 <= Node.val <= 99
 * 	The depth of the tree will be in the range [1, 10].
 * 
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/print-binary-tree/
// discuss: https://leetcode.com/problems/print-binary-tree/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn print_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<String>> {
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_655() {
    }
}
