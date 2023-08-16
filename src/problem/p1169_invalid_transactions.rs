/**
 * [1169] Invalid Transactions
 *
 * A transaction is possibly invalid if:
 * 
 * 	the amount exceeds $1000, or;
 * 	if it occurs within (and including) 60 minutes of another transaction with the same name in a different city.
 * 
 * You are given an array of strings transaction where transactions[i] consists of comma-separated values representing the name, time (in minutes), amount, and city of the transaction.
 * Return a list of transactions that are possibly invalid. You may return the answer in any order.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: transactions = ["alice,20,800,mtv","alice,50,100,beijing"]
 * Output: ["alice,20,800,mtv","alice,50,100,beijing"]
 * Explanation: The first transaction is invalid because the second transaction occurs within a difference of 60 minutes, have the same name and is in a different city. Similarly the second one is invalid too.
 * <strong class="example">Example 2:
 * 
 * Input: transactions = ["alice,20,800,mtv","alice,50,1200,mtv"]
 * Output: ["alice,50,1200,mtv"]
 * 
 * <strong class="example">Example 3:
 * 
 * Input: transactions = ["alice,20,800,mtv","bob,50,1200,mtv"]
 * Output: ["bob,50,1200,mtv"]
 * 
 *  
 * Constraints:
 * 
 * 	transactions.length <= 1000
 * 	Each transactions[i] takes the form "{name},{time},{amount},{city}"
 * 	Each {name} and {city} consist of lowercase English letters, and have lengths between 1 and 10.
 * 	Each {time} consist of digits, and represent an integer between 0 and 1000.
 * 	Each {amount} consist of digits, and represent an integer between 0 and 2000.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/invalid-transactions/
// discuss: https://leetcode.com/problems/invalid-transactions/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn invalid_transactions(transactions: Vec<String>) -> Vec<String> {
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1169() {
    }
}
