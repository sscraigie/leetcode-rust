/**
 * [50] Pow(x, n)
 *
 * Implement <a href="http://www.cplusplus.com/reference/valarray/pow/" target="_blank">pow(x, n)</a>, which calculates x raised to the power n (i.e., x^n).
 *  
 * <strong class="example">Example 1:
 * 
 * Input: x = 2.00000, n = 10
 * Output: 1024.00000
 * 
 * <strong class="example">Example 2:
 * 
 * Input: x = 2.10000, n = 3
 * Output: 9.26100
 * 
 * <strong class="example">Example 3:
 * 
 * Input: x = 2.00000, n = -2
 * Output: 0.25000
 * Explanation: 2^-2 = 1/2^2 = 1/4 = 0.25
 * 
 *  
 * Constraints:
 * 
 * 	-100.0 < x < 100.0
 * 	-2^31 <= n <= 2^31-1
 * 	n is an integer.
 * 	Either x is not zero or n > 0.
 * 	-10^4 <= x^n <= 10^4
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/powx-n/
// discuss: https://leetcode.com/problems/powx-n/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        0f64
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_50() {
    }
}
