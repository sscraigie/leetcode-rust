/**
 * [19] Remove Nth Node From End of List
 *
 * Given the head of a linked list, remove the n^th node from the end of the list and return its head.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/03/remove_ex1.jpg" style="width: 542px; height: 222px;" />
 * Input: head = [1,2,3,4,5], n = 2
 * Output: [1,2,3,5]
 * 
 * <strong class="example">Example 2:
 * 
 * Input: head = [1], n = 1
 * Output: []
 * 
 * <strong class="example">Example 3:
 * 
 * Input: head = [1,2], n = 1
 * Output: [1]
 * 
 *  
 * Constraints:
 * 
 * 	The number of nodes in the list is sz.
 * 	1 <= sz <= 30
 * 	0 <= Node.val <= 100
 * 	1 <= n <= sz
 * 
 *  
 * Follow up: Could you do this in one pass?
 * 
 */
pub struct Solution {}
use crate::util::linked_list::{ListNode, to_list};

// problem: https://leetcode.com/problems/remove-nth-node-from-end-of-list/
// discuss: https://leetcode.com/problems/remove-nth-node-from-end-of-list/discuss/?currentPage=1&orderBy=most_votes&query=

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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        Some(Box::new(ListNode::new(0)))
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_19() {
    }
}
