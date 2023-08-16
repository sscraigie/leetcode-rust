/**
 * [777] Swap Adjacent in LR String
 *
 * In a string composed of 'L', 'R', and 'X' characters, like "RXXLRXRXL", a move consists of either replacing one occurrence of "XL" with "LX", or replacing one occurrence of "RX" with "XR". Given the starting string start and the ending string end, return True if and only if there exists a sequence of moves to transform one string to the other.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: start = "RXXLRXRXL", end = "XRLXXRRLX"
 * Output: true
 * Explanation: We can transform start to end following these steps:
 * RXXLRXRXL ->
 * XRXLRXRXL ->
 * XRLXRXRXL ->
 * XRLXXRRXL ->
 * XRLXXRRLX
 * 
 * <strong class="example">Example 2:
 * 
 * Input: start = "X", end = "L"
 * Output: false
 * 
 *  
 * Constraints:
 * 
 * 	1 <= start.length <= 10^4
 * 	start.length == end.length
 * 	Both start and end will only consist of characters in 'L', 'R', and 'X'.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/swap-adjacent-in-lr-string/
// discuss: https://leetcode.com/problems/swap-adjacent-in-lr-string/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn can_transform(start: String, end: String) -> bool {
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_777() {
    }
}
