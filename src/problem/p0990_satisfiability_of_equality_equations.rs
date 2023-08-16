/**
 * [990] Satisfiability of Equality Equations
 *
 * You are given an array of strings equations that represent relationships between variables where each string equations[i] is of length 4 and takes one of two different forms: "xi==yi" or "xi!=yi".Here, xi and yi are lowercase letters (not necessarily different) that represent one-letter variable names.
 * Return true if it is possible to assign integers to variable names so as to satisfy all the given equations, or false otherwise.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: equations = ["a==b","b!=a"]
 * Output: false
 * Explanation: If we assign say, a = 1 and b = 1, then the first equation is satisfied, but not the second.
 * There is no way to assign the variables to satisfy both equations.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: equations = ["b==a","a==b"]
 * Output: true
 * Explanation: We could assign a = 1 and b = 1 to satisfy both equations.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= equations.length <= 500
 * 	equations[i].length == 4
 * 	equations[i][0] is a lowercase letter.
 * 	equations[i][1] is either '=' or '!'.
 * 	equations[i][2] is '='.
 * 	equations[i][3] is a lowercase letter.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/satisfiability-of-equality-equations/
// discuss: https://leetcode.com/problems/satisfiability-of-equality-equations/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn equations_possible(equations: Vec<String>) -> bool {
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_990() {
    }
}
