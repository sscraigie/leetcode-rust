/**
 * [1359] Count All Valid Pickup and Delivery Options
 *
 * Given n orders, each order consist in pickup and delivery services. 
 * Count all valid pickup/delivery possible sequences such that delivery(i) is always after of pickup(i). 
 * Since the answer may be too large, return it modulo 10^9 + 7.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: n = 1
 * Output: 1
 * Explanation: Unique order (P1, D1), Delivery 1 always is after of Pickup 1.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: n = 2
 * Output: 6
 * Explanation: All possible orders: 
 * (P1,P2,D1,D2), (P1,P2,D2,D1), (P1,D1,P2,D2), (P2,P1,D1,D2), (P2,P1,D2,D1) and (P2,D2,P1,D1).
 * This is an invalid order (P1,D2,P2,D1) because Pickup 2 is after of Delivery 2.
 * 
 * <strong class="example">Example 3:
 * 
 * Input: n = 3
 * Output: 90
 * 
 *  
 * Constraints:
 * 
 * 	1 <= n <= 500
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-all-valid-pickup-and-delivery-options/
// discuss: https://leetcode.com/problems/count-all-valid-pickup-and-delivery-options/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_orders(n: i32) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1359() {
    }
}
