/**
 * [592] Fraction Addition and Subtraction
 *
 * Given a string expression representing an expression of fraction addition and subtraction, return the calculation result in string format.
 * The final result should be an <a href="https://en.wikipedia.org/wiki/Irreducible_fraction" target="_blank">irreducible fraction</a>. If your final result is an integer, change it to the format of a fraction that has a denominator 1. So in this case, 2 should be converted to 2/1.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: expression = "-1/2+1/2"
 * Output: "0/1"
 * 
 * <strong class="example">Example 2:
 * 
 * Input: expression = "-1/2+1/2+1/3"
 * Output: "1/3"
 * 
 * <strong class="example">Example 3:
 * 
 * Input: expression = "1/3-1/2"
 * Output: "-1/6"
 * 
 *  
 * Constraints:
 * 
 * 	The input string only contains '0' to '9', '/', '+' and '-'. So does the output.
 * 	Each fraction (input and output) has the format &plusmn;numerator/denominator. If the first input fraction or the output is positive, then '+' will be omitted.
 * 	The input only contains valid irreducible fractions, where the numerator and denominator of each fraction will always be in the range [1, 10]. If the denominator is 1, it means this fraction is actually an integer in a fraction format defined above.
 * 	The number of given fractions will be in the range [1, 10].
 * 	The numerator and denominator of the final result are guaranteed to be valid and in the range of 32-bit int.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/fraction-addition-and-subtraction/
// discuss: https://leetcode.com/problems/fraction-addition-and-subtraction/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn fraction_addition(expression: String) -> String {
        String::new()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_592() {
    }
}
