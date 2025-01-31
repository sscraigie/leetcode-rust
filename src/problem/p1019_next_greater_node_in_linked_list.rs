/**
 * [1019] Next Greater Node In Linked List
 *
 * You are given the head of a linked list with n nodes.
 * For each node in the list, find the value of the next greater node. That is, for each node, find the value of the first node that is next to it and has a strictly larger value than it.
 * Return an integer array answer where answer[i] is the value of the next greater node of the i^th node (1-indexed). If the i^th node does not have a next greater node, set answer[i] = 0.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/08/05/linkedlistnext1.jpg" style="width: 304px; height: 133px;" />
 * Input: head = [2,1,5]
 * Output: [5,5,0]
 * 
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/08/05/linkedlistnext2.jpg" style="width: 500px; height: 113px;" />
 * Input: head = [2,7,4,3,5]
 * Output: [7,0,5,5,0]
 * 
 *  
 * Constraints:
 * 
 * 	The number of nodes in the list is n.
 * 	1 <= n <= 10^4
 * 	1 <= Node.val <= 10^9
 * 
 */
pub struct Solution {}
use crate::util::linked_list::{ListNode, to_list};

// problem: https://leetcode.com/problems/next-greater-node-in-linked-list/
// discuss: https://leetcode.com/problems/next-greater-node-in-linked-list/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn next_larger_nodes(head: Option<Box<ListNode>>) -> Vec<i32> {
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1019() {
    }
}
