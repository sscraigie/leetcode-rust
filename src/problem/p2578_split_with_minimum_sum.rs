/**
 * [2578] Split With Minimum Sum
 *
 * Given a positive integer num, split it into two non-negative integers num1 and num2 such that:
 * 
 * 	The concatenation of num1 and num2 is a permutation of num.
 * 	
 * 		In other words, the sum of the number of occurrences of each digit in num1 and num2 is equal to the number of occurrences of that digit in num.
 * 	
 * 	
 * 	num1 and num2 can contain leading zeros.
 * 
 * Return the minimum possible sum of num1 and num2.
 * Notes:
 * 
 * 	It is guaranteed that num does not contain any leading zeros.
 * 	The order of occurrence of the digits in num1 and num2 may differ from the order of occurrence of num.
 * 
 *  
 * <strong class="example">Example 1:
 * 
 * Input: num = 4325
 * Output: 59
 * Explanation: We can split 4325 so that num1 is 24 and num2 is 35, giving a sum of 59. We can prove that 59 is indeed the minimal possible sum.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: num = 687
 * Output: 75
 * Explanation: We can split 687 so that num1 is 68 and num2 is 7, which would give an optimal sum of 75.
 * 
 *  
 * Constraints:
 * 
 * 	10 <= num <= 10^9
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/split-with-minimum-sum/
// discuss: https://leetcode.com/problems/split-with-minimum-sum/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn split_num(num: i32) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2578() {
    }
}
