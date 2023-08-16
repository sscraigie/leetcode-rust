/**
 * [1014] Best Sightseeing Pair
 *
 * You are given an integer array values where values[i] represents the value of the i^th sightseeing spot. Two sightseeing spots i and j have a distance j - i between them.
 * The score of a pair (i < j) of sightseeing spots is values[i] + values[j] + i - j: the sum of the values of the sightseeing spots, minus the distance between them.
 * Return the maximum score of a pair of sightseeing spots.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: values = [8,1,5,2,6]
 * Output: 11
 * Explanation: i = 0, j = 2, values[i] + values[j] + i - j = 8 + 5 + 0 - 2 = 11
 * 
 * <strong class="example">Example 2:
 * 
 * Input: values = [1,2]
 * Output: 2
 * 
 *  
 * Constraints:
 * 
 * 	2 <= values.length <= 5 * 10^4
 * 	1 <= values[i] <= 1000
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/best-sightseeing-pair/
// discuss: https://leetcode.com/problems/best-sightseeing-pair/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1014() {
    }
}
