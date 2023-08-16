/**
 * [2545] Sort the Students by Their Kth Score
 *
 * There is a class with m students and n exams. You are given a 0-indexed m x n integer matrix score, where each row represents one student and score[i][j] denotes the score the i^th student got in the j^th exam. The matrix score contains distinct integers only.
 * You are also given an integer k. Sort the students (i.e., the rows of the matrix) by their scores in the k^th (0-indexed) exam from the highest to the lowest.
 * Return the matrix after sorting it.
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/11/30/example1.png" style="width: 600px; height: 136px;" />
 * Input: score = [[10,6,9,1],[7,5,11,2],[4,8,3,15]], k = 2
 * Output: [[7,5,11,2],[10,6,9,1],[4,8,3,15]]
 * Explanation: In the above diagram, S denotes the student, while E denotes the exam.
 * - The student with index 1 scored 11 in exam 2, which is the highest score, so they got first place.
 * - The student with index 0 scored 9 in exam 2, which is the second highest score, so they got second place.
 * - The student with index 2 scored 3 in exam 2, which is the lowest score, so they got third place.
 * 
 * <strong class="example">Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2022/11/30/example2.png" style="width: 486px; height: 121px;" />
 * Input: score = [[3,4],[5,6]], k = 0
 * Output: [[5,6],[3,4]]
 * Explanation: In the above diagram, S denotes the student, while E denotes the exam.
 * - The student with index 1 scored 5 in exam 0, which is the highest score, so they got first place.
 * - The student with index 0 scored 3 in exam 0, which is the lowest score, so they got second place.
 * 
 *  
 * Constraints:
 * 
 * 	m == score.length
 * 	n == score[i].length
 * 	1 <= m, n <= 250
 * 	1 <= score[i][j] <= 10^5
 * 	score consists of distinct integers.
 * 	0 <= k < n
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sort-the-students-by-their-kth-score/
// discuss: https://leetcode.com/problems/sort-the-students-by-their-kth-score/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn sort_the_students(score: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2545() {
    }
}
