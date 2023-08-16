/**
 * [2264] Largest 3-Same-Digit Number in String
 *
 * You are given a string num representing a large integer. An integer is good if it meets the following conditions:
 * 
 * 	It is a substring of num with length 3.
 * 	It consists of only one unique digit.
 * 
 * Return the maximum good integer as a string or an empty string "" if no such integer exists.
 * Note:
 * 
 * 	A substring is a contiguous sequence of characters within a string.
 * 	There may be leading zeroes in num or a good integer.
 * 
 *  
 * <strong class="example">Example 1:
 * 
 * Input: num = "6<u>777</u>133339"
 * Output: "777"
 * Explanation: There are two distinct good integers: "777" and "333".
 * "777" is the largest, so we return "777".
 * 
 * <strong class="example">Example 2:
 * 
 * Input: num = "23<u>000</u>19"
 * Output: "000"
 * Explanation: "000" is the only good integer.
 * 
 * <strong class="example">Example 3:
 * 
 * Input: num = "42352338"
 * Output: ""
 * Explanation: No substring of length 3 consists of only one unique digit. Therefore, there are no good integers.
 * 
 *  
 * Constraints:
 * 
 * 	3 <= num.length <= 1000
 * 	num only consists of digits.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/largest-3-same-digit-number-in-string/
// discuss: https://leetcode.com/problems/largest-3-same-digit-number-in-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        String::new()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2264() {
    }
}
