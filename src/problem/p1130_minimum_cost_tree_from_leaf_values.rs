/**
 * [1130] Minimum Cost Tree From Leaf Values
 *
 * Given an array arr of positive integers, consider all binary trees such that:
 * 
 * 	Each node has either 0 or 2 children;
 * 	The values of arr correspond to the values of each leaf in an in-order traversal of the tree.
 * 	The value of each non-leaf node is equal to the product of the largest leaf value in its left and right subtree, respectively.
 * 
 * Among all possible binary trees considered, return the smallest possible sum of the values of each non-leaf node. It is guaranteed this sum fits into a 32-bit integer.
 * A node is a leaf if and only if it has zero children.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/08/10/tree1.jpg" style="width: 500px; height: 169px;" />
 * Input: arr = [6,2,4]
 * Output: 32
 * Explanation: There are two possible trees shown.
 * The first has a non-leaf node sum 36, and the second has non-leaf node sum 32.
 * 
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/08/10/tree2.jpg" style="width: 224px; height: 145px;" />
 * Input: arr = [4,11]
 * Output: 44
 * 
 *  
 * Constraints:
 * 
 * 	2 <= arr.length <= 40
 * 	1 <= arr[i] <= 15
 * 	It is guaranteed that the answer fits into a 32-bit signed integer (i.e., it is less than 2^31).
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-cost-tree-from-leaf-values/
// discuss: https://leetcode.com/problems/minimum-cost-tree-from-leaf-values/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn mct_from_leaf_values(arr: Vec<i32>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1130() {
    }
}
