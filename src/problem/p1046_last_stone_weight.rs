/**
 * [1046] Last Stone Weight
 *
 * You are given an array of integers stones where stones[i] is the weight of the i^th stone.
 * We are playing a game with the stones. On each turn, we choose the heaviest two stones and smash them together. Suppose the heaviest two stones have weights x and y with x <= y. The result of this smash is:
 * 
 * 	If x == y, both stones are destroyed, and
 * 	If x != y, the stone of weight x is destroyed, and the stone of weight y has new weight y - x.
 * 
 * At the end of the game, there is at most one stone left.
 * Return the weight of the last remaining stone. If there are no stones left, return 0.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: stones = [2,7,4,1,8,1]
 * Output: 1
 * Explanation: 
 * We combine 7 and 8 to get 1 so the array converts to [2,4,1,1,1] then,
 * we combine 2 and 4 to get 2 so the array converts to [2,1,1,1] then,
 * we combine 2 and 1 to get 1 so the array converts to [1,1,1] then,
 * we combine 1 and 1 to get 0 so the array converts to [1] then that's the value of the last stone.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: stones = [1]
 * Output: 1
 * 
 *  
 * Constraints:
 * 
 * 	1 <= stones.length <= 30
 * 	1 <= stones[i] <= 1000
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/last-stone-weight/
// discuss: https://leetcode.com/problems/last-stone-weight/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1046() {
    }
}
