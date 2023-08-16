/**
 * [2641] Cousins in Binary Tree II
 *
 * Given the root of a binary tree, replace the value of each node in the tree with the sum of all its cousins' values.
 * Two nodes of a binary tree are cousins if they have the same depth with different parents.
 * Return the root of the modified tree.
 * Note that the depth of a node is the number of edges in the path from the root node to it.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2023/01/11/example11.png" style="width: 571px; height: 151px;" />
 * Input: root = [5,4,9,1,10,null,7]
 * Output: [0,0,0,7,7,null,11]
 * Explanation: The diagram above shows the initial binary tree and the binary tree after changing the value of each node.
 * - Node with value 5 does not have any cousins so its sum is 0.
 * - Node with value 4 does not have any cousins so its sum is 0.
 * - Node with value 9 does not have any cousins so its sum is 0.
 * - Node with value 1 has a cousin with value 7 so its sum is 7.
 * - Node with value 10 has a cousin with value 7 so its sum is 7.
 * - Node with value 7 has cousins with values 1 and 10 so its sum is 11.
 * 
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2023/01/11/diagram33.png" style="width: 481px; height: 91px;" />
 * Input: root = [3,1,2]
 * Output: [0,0,0]
 * Explanation: The diagram above shows the initial binary tree and the binary tree after changing the value of each node.
 * - Node with value 3 does not have any cousins so its sum is 0.
 * - Node with value 1 does not have any cousins so its sum is 0.
 * - Node with value 2 does not have any cousins so its sum is 0.
 * 
 *  
 * Constraints:
 * 
 * 	The number of nodes in the tree is in the range [1, 10^5].
 * 	1 <= Node.val <= 10^4
 * 
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/cousins-in-binary-tree-ii/
// discuss: https://leetcode.com/problems/cousins-in-binary-tree-ii/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn replace_value_in_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode::new(0))))
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2641() {
    }
}
