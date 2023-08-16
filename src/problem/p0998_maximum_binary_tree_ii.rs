/**
 * [998] Maximum Binary Tree II
 *
 * A maximum tree is a tree where every node has a value greater than any other value in its subtree.
 * You are given the root of a maximum binary tree and an integer val.
 * Just as in the <a href="https://leetcode.com/problems/maximum-binary-tree/" target="_blank">previous problem</a>, the given tree was constructed from a list a (root = Construct(a)) recursively with the following Construct(a) routine:
 * 
 * 	If a is empty, return null.
 * 	Otherwise, let a[i] be the largest element of a. Create a root node with the value a[i].
 * 	The left child of root will be Construct([a[0], a[1], ..., a[i - 1]]).
 * 	The right child of root will be Construct([a[i + 1], a[i + 2], ..., a[a.length - 1]]).
 * 	Return root.
 * 
 * Note that we were not given a directly, only a root node root = Construct(a).
 * Suppose b is a copy of a with the value val appended to it. It is guaranteed that b has unique values.
 * Return Construct(b).
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/08/09/maxtree1.JPG" style="width: 376px; height: 235px;" />
 * Input: root = [4,1,3,null,null,2], val = 5
 * Output: [5,4,null,1,3,null,null,2]
 * Explanation: a = [1,4,2,3], b = [1,4,2,3,5]
 * 
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/08/09/maxtree21.JPG" style="width: 358px; height: 156px;" />
 * Input: root = [5,2,4,null,1], val = 3
 * Output: [5,2,4,null,1,null,3]
 * Explanation: a = [2,1,5,4], b = [2,1,5,4,3]
 * 
 * <strong class="example">Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/08/09/maxtree3.JPG" style="width: 404px; height: 180px;" />
 * Input: root = [5,2,3,null,1], val = 4
 * Output: [5,2,4,null,1,3]
 * Explanation: a = [2,1,5,3], b = [2,1,5,3,4]
 * 
 *  
 * Constraints:
 * 
 * 	The number of nodes in the tree is in the range [1, 100].
 * 	1 <= Node.val <= 100
 * 	All the values of the tree are unique.
 * 	1 <= val <= 100
 * 
 */
pub struct Solution {}
use crate::util::tree::{TreeNode, to_tree};

// problem: https://leetcode.com/problems/maximum-binary-tree-ii/
// discuss: https://leetcode.com/problems/maximum-binary-tree-ii/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn insert_into_max_tree(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode::new(0))))
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_998() {
    }
}
