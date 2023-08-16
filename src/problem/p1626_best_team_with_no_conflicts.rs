/**
 * [1626] Best Team With No Conflicts
 *
 * You are the manager of a basketball team. For the upcoming tournament, you want to choose the team with the highest overall score. The score of the team is the sum of scores of all the players in the team.
 * However, the basketball team is not allowed to have conflicts. A conflict exists if a younger player has a strictly higher score than an older player. A conflict does not occur between players of the same age.
 * Given two lists, scores and ages, where each scores[i] and ages[i] represents the score and age of the i^th player, respectively, return the highest overall score of all possible basketball teams.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: scores = [1,3,5,10,15], ages = [1,2,3,4,5]
 * Output: 34
 * Explanation: You can choose all the players.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: scores = [4,5,6,5], ages = [2,1,2,1]
 * Output: 16
 * Explanation: It is best to choose the last 3 players. Notice that you are allowed to choose multiple people of the same age.
 * 
 * <strong class="example">Example 3:
 * 
 * Input: scores = [1,2,3,5], ages = [8,9,10,1]
 * Output: 6
 * Explanation: It is best to choose the first 3 players. 
 * 
 *  
 * Constraints:
 * 
 * 	1 <= scores.length, ages.length <= 1000
 * 	scores.length == ages.length
 * 	1 <= scores[i] <= 10^6
 * 	1 <= ages[i] <= 1000
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/best-team-with-no-conflicts/
// discuss: https://leetcode.com/problems/best-team-with-no-conflicts/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn best_team_score(scores: Vec<i32>, ages: Vec<i32>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1626() {
    }
}