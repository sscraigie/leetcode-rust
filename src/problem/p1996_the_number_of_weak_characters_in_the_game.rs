/**
 * [1996] The Number of Weak Characters in the Game
 *
 * You are playing a game that contains multiple characters, and each of the characters has two main properties: attack and defense. You are given a 2D integer array properties where properties[i] = [attacki, defensei] represents the properties of the i^th character in the game.
 * A character is said to be weak if any other character has both attack and defense levels strictly greater than this character's attack and defense levels. More formally, a character i is said to be weak if there exists another character j where attackj > attacki and defensej > defensei.
 * Return the number of weak characters.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: properties = [[5,5],[6,3],[3,6]]
 * Output: 0
 * Explanation: No character has strictly greater attack and defense than the other.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: properties = [[2,2],[3,3]]
 * Output: 1
 * Explanation: The first character is weak because the second character has a strictly greater attack and defense.
 * 
 * <strong class="example">Example 3:
 * 
 * Input: properties = [[1,5],[10,4],[4,3]]
 * Output: 1
 * Explanation: The third character is weak because the second character has a strictly greater attack and defense.
 * 
 *  
 * Constraints:
 * 
 * 	2 <= properties.length <= 10^5
 * 	properties[i].length == 2
 * 	1 <= attacki, defensei <= 10^5
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/the-number-of-weak-characters-in-the-game/
// discuss: https://leetcode.com/problems/the-number-of-weak-characters-in-the-game/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn number_of_weak_characters(properties: Vec<Vec<i32>>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1996() {
    }
}
