/**
 * [166] Fraction to Recurring Decimal
 *
 * Given two integers representing the numerator and denominator of a fraction, return the fraction in string format.
 * If the fractional part is repeating, enclose the repeating part in parentheses.
 * If multiple answers are possible, return any of them.
 * It is guaranteed that the length of the answer string is less than 10^4 for all the given inputs.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: numerator = 1, denominator = 2
 * Output: "0.5"
 * 
 * <strong class="example">Example 2:
 * 
 * Input: numerator = 2, denominator = 1
 * Output: "2"
 * 
 * <strong class="example">Example 3:
 * 
 * Input: numerator = 4, denominator = 333
 * Output: "0.(012)"
 * 
 *  
 * Constraints:
 * 
 * 	-2^31 <= numerator, denominator <= 2^31 - 1
 * 	denominator != 0
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/fraction-to-recurring-decimal/
// discuss: https://leetcode.com/problems/fraction-to-recurring-decimal/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        String::new()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_166() {
    }
}
