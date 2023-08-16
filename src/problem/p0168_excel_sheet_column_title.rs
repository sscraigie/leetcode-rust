/**
 * [168] Excel Sheet Column Title
 *
 * Given an integer columnNumber, return its corresponding column title as it appears in an Excel sheet.
 * For example:
 * 
 * A -> 1
 * B -> 2
 * C -> 3
 * ...
 * Z -> 26
 * AA -> 27
 * AB -> 28 
 * ...
 * 
 *  
 * <strong class="example">Example 1:
 * 
 * Input: columnNumber = 1
 * Output: "A"
 * 
 * <strong class="example">Example 2:
 * 
 * Input: columnNumber = 28
 * Output: "AB"
 * 
 * <strong class="example">Example 3:
 * 
 * Input: columnNumber = 701
 * Output: "ZY"
 * 
 *  
 * Constraints:
 * 
 * 	1 <= columnNumber <= 2^31 - 1
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/excel-sheet-column-title/
// discuss: https://leetcode.com/problems/excel-sheet-column-title/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
        String::new()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_168() {
    }
}
