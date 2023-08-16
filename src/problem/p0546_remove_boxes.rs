/**
 * [546] Remove Boxes
 *
 * You are given several boxes with different colors represented by different positive numbers.
 * You may experience several rounds to remove boxes until there is no box left. Each time you can choose some continuous boxes with the same color (i.e., composed of k boxes, k >= 1), remove them and get k * k points.
 * Return the maximum points you can get.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: boxes = [1,3,2,2,2,3,4,3,1]
 * Output: 23
 * Explanation:
 * [1, 3, 2, 2, 2, 3, 4, 3, 1] 
 * ----> [1, 3, 3, 4, 3, 1] (3*3=9 points) 
 * ----> [1, 3, 3, 3, 1] (1*1=1 points) 
 * ----> [1, 1] (3*3=9 points) 
 * ----> [] (2*2=4 points)
 * 
 * <strong class="example">Example 2:
 * 
 * Input: boxes = [1,1,1]
 * Output: 9
 * 
 * <strong class="example">Example 3:
 * 
 * Input: boxes = [1]
 * Output: 1
 * 
 *  
 * Constraints:
 * 
 * 	1 <= boxes.length <= 100
 * 	1 <= boxes[i] <= 100
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/remove-boxes/
// discuss: https://leetcode.com/problems/remove-boxes/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn remove_boxes(boxes: Vec<i32>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_546() {
    }
}
