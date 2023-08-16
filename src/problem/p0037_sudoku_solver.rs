/**
 * [37] Sudoku Solver
 *
 * Write a program to solve a Sudoku puzzle by filling the empty cells.
 * A sudoku solution must satisfy all of the following rules:
 * <ol>
 * 	Each of the digits 1-9 must occur exactly once in each row.
 * 	Each of the digits 1-9 must occur exactly once in each column.
 * 	Each of the digits 1-9 must occur exactly once in each of the 9 3x3 sub-boxes of the grid.
 * </ol>
 * The '.' character indicates empty cells.
 *  
 * <strong class="example">Example 1:
 * <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/f/ff/Sudoku-by-L2G-20050714.svg/250px-Sudoku-by-L2G-20050714.svg.png" style="height:250px; width:250px" />
 * Input: board = [["5","3",".",".","7",".",".",".","."],["6",".",".","1","9","5",".",".","."],[".","9","8",".",".",".",".","6","."],["8",".",".",".","6",".",".",".","3"],["4",".",".","8",".","3",".",".","1"],["7",".",".",".","2",".",".",".","6"],[".","6",".",".",".",".","2","8","."],[".",".",".","4","1","9",".",".","5"],[".",".",".",".","8",".",".","7","9"]]
 * Output: [["5","3","4","6","7","8","9","1","2"],["6","7","2","1","9","5","3","4","8"],["1","9","8","3","4","2","5","6","7"],["8","5","9","7","6","1","4","2","3"],["4","2","6","8","5","3","7","9","1"],["7","1","3","9","2","4","8","5","6"],["9","6","1","5","3","7","2","8","4"],["2","8","7","4","1","9","6","3","5"],["3","4","5","2","8","6","1","7","9"]]
 * Explanation: The input board is shown above and the only valid solution is shown below:
 * <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/3/31/Sudoku-by-L2G-20050714_solution.svg/250px-Sudoku-by-L2G-20050714_solution.svg.png" style="height:250px; width:250px" />
 * 
 *  
 * Constraints:
 * 
 * 	board.length == 9
 * 	board[i].length == 9
 * 	board[i][j] is a digit or '.'.
 * 	It is guaranteed that the input board has only one solution.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sudoku-solver/
// discuss: https://leetcode.com/problems/sudoku-solver/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_37() {
    }
}