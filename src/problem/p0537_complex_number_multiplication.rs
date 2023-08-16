/**
 * [537] Complex Number Multiplication
 *
 * A <a href="https://en.wikipedia.org/wiki/Complex_number" target="_blank">complex number</a> can be represented as a string on the form "real+imaginaryi" where:
 * 
 * 	real is the real part and is an integer in the range [-100, 100].
 * 	imaginary is the imaginary part and is an integer in the range [-100, 100].
 * 	i^2 == -1.
 * 
 * Given two complex numbers num1 and num2 as strings, return a string of the complex number that represents their multiplications.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: num1 = "1+1i", num2 = "1+1i"
 * Output: "0+2i"
 * Explanation: (1 + i) * (1 + i) = 1 + i2 + 2 * i = 2i, and you need convert it to the form of 0+2i.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: num1 = "1+-1i", num2 = "1+-1i"
 * Output: "0+-2i"
 * Explanation: (1 - i) * (1 - i) = 1 + i2 - 2 * i = -2i, and you need convert it to the form of 0+-2i.
 * 
 *  
 * Constraints:
 * 
 * 	num1 and num2 are valid complex numbers.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/complex-number-multiplication/
// discuss: https://leetcode.com/problems/complex-number-multiplication/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn complex_number_multiply(num1: String, num2: String) -> String {
        String::new()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_537() {
    }
}
