/**
 * [991] Broken Calculator
 *
 * There is a broken calculator that has the integer startValue on its display initially. In one operation, you can:
 * 
 * 	multiply the number on display by 2, or
 * 	subtract 1 from the number on display.
 * 
 * Given two integers startValue and target, return the minimum number of operations needed to display target on the calculator.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: startValue = 2, target = 3
 * Output: 2
 * Explanation: Use double operation and then decrement operation {2 -> 4 -> 3}.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: startValue = 5, target = 8
 * Output: 2
 * Explanation: Use decrement and then double {5 -> 4 -> 8}.
 * 
 * <strong class="example">Example 3:
 * 
 * Input: startValue = 3, target = 10
 * Output: 3
 * Explanation: Use double, decrement and double {3 -> 6 -> 5 -> 10}.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= startValue, target <= 10^9
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/broken-calculator/
// discuss: https://leetcode.com/problems/broken-calculator/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn broken_calc(start_value: i32, target: i32) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_991() {
    }
}
