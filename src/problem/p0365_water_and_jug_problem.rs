/**
 * [365] Water and Jug Problem
 *
 * You are given two jugs with capacities jug1Capacity and jug2Capacity liters. There is an infinite amount of water supply available. Determine whether it is possible to measure exactly targetCapacity liters using these two jugs.
 * If targetCapacity liters of water are measurable, you must have targetCapacity liters of water contained within one or both buckets by the end.
 * Operations allowed:
 * 
 * 	Fill any of the jugs with water.
 * 	Empty any of the jugs.
 * 	Pour water from one jug into another till the other jug is completely full, or the first jug itself is empty.
 * 
 *  
 * <strong class="example">Example 1:
 * 
 * Input: jug1Capacity = 3, jug2Capacity = 5, targetCapacity = 4
 * Output: true
 * Explanation: The famous <a href="https://www.youtube.com/watch?v=BVtQNK_ZUJg&amp;ab_channel=notnek01" target="_blank">Die Hard</a> example 
 * 
 * <strong class="example">Example 2:
 * 
 * Input: jug1Capacity = 2, jug2Capacity = 6, targetCapacity = 5
 * Output: false
 * 
 * <strong class="example">Example 3:
 * 
 * Input: jug1Capacity = 1, jug2Capacity = 2, targetCapacity = 3
 * Output: true
 * 
 *  
 * Constraints:
 * 
 * 	1 <= jug1Capacity, jug2Capacity, targetCapacity <= 10^6
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/water-and-jug-problem/
// discuss: https://leetcode.com/problems/water-and-jug-problem/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn can_measure_water(jug1_capacity: i32, jug2_capacity: i32, target_capacity: i32) -> bool {
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_365() {
    }
}
