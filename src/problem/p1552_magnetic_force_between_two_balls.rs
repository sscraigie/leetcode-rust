/**
 * [1552] Magnetic Force Between Two Balls
 *
 * In the universe Earth C-137, Rick discovered a special form of magnetic force between two balls if they are put in his new invented basket. Rick has n empty baskets, the i^th basket is at position[i], Morty has m balls and needs to distribute the balls into the baskets such that the minimum magnetic force between any two balls is maximum.
 * Rick stated that magnetic force between two different balls at positions x and y is |x - y|.
 * Given the integer array position and the integer m. Return the required force.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/08/11/q3v1.jpg" style="width: 562px; height: 195px;" />
 * Input: position = [1,2,3,4,7], m = 3
 * Output: 3
 * Explanation: Distributing the 3 balls into baskets 1, 4 and 7 will make the magnetic force between ball pairs [3, 3, 6]. The minimum magnetic force is 3. We cannot achieve a larger minimum magnetic force than 3.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: position = [5,4,3,2,1,1000000000], m = 2
 * Output: 999999999
 * Explanation: We can use baskets 1 and 1000000000.
 * 
 *  
 * Constraints:
 * 
 * 	n == position.length
 * 	2 <= n <= 10^5
 * 	1 <= position[i] <= 10^9
 * 	All integers in position are distinct.
 * 	2 <= m <= position.length
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/magnetic-force-between-two-balls/
// discuss: https://leetcode.com/problems/magnetic-force-between-two-balls/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_distance(position: Vec<i32>, m: i32) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1552() {
    }
}
