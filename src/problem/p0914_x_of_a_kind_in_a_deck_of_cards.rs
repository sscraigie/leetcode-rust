/**
 * [914] X of a Kind in a Deck of Cards
 *
 * You are given an integer array deck where deck[i] represents the number written on the i^th card.
 * Partition the cards into one or more groups such that:
 * 
 * 	Each group has exactly x cards where x > 1, and
 * 	All the cards in one group have the same integer written on them.
 * 
 * Return true if such partition is possible, or false otherwise.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: deck = [1,2,3,4,4,3,2,1]
 * Output: true
 * Explanation: Possible partition [1,1],[2,2],[3,3],[4,4].
 * 
 * <strong class="example">Example 2:
 * 
 * Input: deck = [1,1,1,2,2,2,3,3]
 * Output: false
 * Explanation: No possible partition.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= deck.length <= 10^4
 * 	0 <= deck[i] < 10^4
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/x-of-a-kind-in-a-deck-of-cards/
// discuss: https://leetcode.com/problems/x-of-a-kind-in-a-deck-of-cards/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn has_groups_size_x(deck: Vec<i32>) -> bool {
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_914() {
    }
}
