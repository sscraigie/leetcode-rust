/**
 * [385] Mini Parser
 *
 * Given a string s represents the serialization of a nested list, implement a parser to deserialize it and return the deserialized NestedInteger.
 * Each element is either an integer or a list whose elements may also be integers or other lists.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: s = "324"
 * Output: 324
 * Explanation: You should return a NestedInteger object which contains a single integer 324.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: s = "[123,[456,[789]]]"
 * Output: [123,[456,[789]]]
 * Explanation: Return a NestedInteger object containing a nested list with 2 elements:
 * 1. An integer containing value 123.
 * 2. A nested list containing two elements:
 *     i.  An integer containing value 456.
 *     ii. A nested list with one element:
 *          a. An integer containing value 789
 * 
 *  
 * Constraints:
 * 
 * 	1 <= s.length <= 5 * 10^4
 * 	s consists of digits, square brackets "[]", negative sign '-', and commas ','.
 * 	s is the serialization of valid NestedInteger.
 * 	All the values in the input are in the range [-10^6, 10^6].
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/mini-parser/
// discuss: https://leetcode.com/problems/mini-parser/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

// #[derive(Debug, PartialEq, Eq)]
// pub enum NestedInteger {
//   Int(i32),
//   List(Vec<NestedInteger>)
// }
impl Solution {
    pub fn deserialize(s: String) -> NestedInteger {
        
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_385() {
    }
}
