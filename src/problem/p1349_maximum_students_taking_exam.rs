/**
 * [1349] Maximum Students Taking Exam
 *
 * Given a m * n matrix seats  that represent seats distributions in a classroom. If a seat is broken, it is denoted by '#' character otherwise it is denoted by a '.' character.
 * Students can see the answers of those sitting next to the left, right, upper left and upper right, but he cannot see the answers of the student sitting directly in front or behind him. Return the maximum number of students that can take the exam together without any cheating being possible..
 * Students must be placed in seats in good condition.
 *  
 * <strong class="example">Example 1:
 * <img height="200" src="https://assets.leetcode.com/uploads/2020/01/29/image.png" width="339" />
 * Input: seats = [["#",".","#","#",".","#"],
 *                 [".","#","#","#","#","."],
 *                 ["#",".","#","#",".","#"]]
 * Output: 4
 * Explanation: Teacher can place 4 students in available seats so they don't cheat on the exam. 
 * 
 * <strong class="example">Example 2:
 * 
 * Input: seats = [[".","#"],
 *                 ["#","#"],
 *                 ["#","."],
 *                 ["#","#"],
 *                 [".","#"]]
 * Output: 3
 * Explanation: Place all students in available seats. 
 * 
 * <strong class="example">Example 3:
 * 
 * Input: seats = [["#",".",".",".","#"],
 *                 [".","#",".","#","."],
 *                 [".",".","#",".","."],
 *                 [".","#",".","#","."],
 *                 ["#",".",".",".","#"]]
 * Output: 10
 * Explanation: Place students in available seats in column 1, 3 and 5.
 * 
 *  
 * Constraints:
 * 
 * 	seats contains only characters '.'<font face="sans-serif, Arial, Verdana, Trebuchet MS"> and</font>'#'.
 * 	m == seats.length
 * 	n == seats[i].length
 * 	1 <= m <= 8
 * 	1 <= n <= 8
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-students-taking-exam/
// discuss: https://leetcode.com/problems/maximum-students-taking-exam/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_students(seats: Vec<Vec<char>>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1349() {
    }
}
