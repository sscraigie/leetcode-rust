/**
 * [838] Push Dominoes
 *
 * There are n dominoes in a line, and we place each domino vertically upright. In the beginning, we simultaneously push some of the dominoes either to the left or to the right.
 * After each second, each domino that is falling to the left pushes the adjacent domino on the left. Similarly, the dominoes falling to the right push their adjacent dominoes standing on the right.
 * When a vertical domino has dominoes falling on it from both sides, it stays still due to the balance of the forces.
 * For the purposes of this question, we will consider that a falling domino expends no additional force to a falling or already fallen domino.
 * You are given a string dominoes representing the initial state where:
 * 
 * 	dominoes[i] = 'L', if the i^th domino has been pushed to the left,
 * 	dominoes[i] = 'R', if the i^th domino has been pushed to the right, and
 * 	dominoes[i] = '.', if the i^th domino has not been pushed.
 * 
 * Return a string representing the final state.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: dominoes = "RR.L"
 * Output: "RR.L"
 * Explanation: The first domino expends no additional force on the second domino.
 * 
 * <strong class="example">Example 2:
 * <img alt="" src="https://s3-lc-upload.s3.amazonaws.com/uploads/2018/05/18/domino.png" style="height: 196px; width: 512px;" />
 * Input: dominoes = ".L.R...LR..L.."
 * Output: "LL.RR.LLRRLL.."
 * 
 *  
 * Constraints:
 * 
 * 	n == dominoes.length
 * 	1 <= n <= 10^5
 * 	dominoes[i] is either 'L', 'R', or '.'.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/push-dominoes/
// discuss: https://leetcode.com/problems/push-dominoes/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        String::new()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_838() {
    }
}
