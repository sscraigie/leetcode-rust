/**
 * [1773] Count Items Matching a Rule
 *
 * You are given an array items, where each items[i] = [typei, colori, namei] describes the type, color, and name of the i^th item. You are also given a rule represented by two strings, ruleKey and ruleValue.
 * The i^th item is said to match the rule if one of the following is true:
 * 
 * 	ruleKey == "type" and ruleValue == typei.
 * 	ruleKey == "color" and ruleValue == colori.
 * 	ruleKey == "name" and ruleValue == namei.
 * 
 * Return the number of items that match the given rule.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: items = [["phone","blue","pixel"],["computer","silver","lenovo"],["phone","gold","iphone"]], ruleKey = "color", ruleValue = "silver"
 * Output: 1
 * Explanation: There is only one item matching the given rule, which is ["computer","silver","lenovo"].
 * 
 * <strong class="example">Example 2:
 * 
 * Input: items = [["phone","blue","pixel"],["computer","silver","phone"],["phone","gold","iphone"]], ruleKey = "type", ruleValue = "phone"
 * Output: 2
 * Explanation: There are only two items matching the given rule, which are ["phone","blue","pixel"] and ["phone","gold","iphone"]. Note that the item ["computer","silver","phone"] does not match.
 *  
 * Constraints:
 * 
 * 	1 <= items.length <= 10^4
 * 	1 <= typei.length, colori.length, namei.length, ruleValue.length <= 10
 * 	ruleKey is equal to either "type", "color", or "name".
 * 	All strings consist only of lowercase letters.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-items-matching-a-rule/
// discuss: https://leetcode.com/problems/count-items-matching-a-rule/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1773() {
    }
}
