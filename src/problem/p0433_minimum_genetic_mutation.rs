/**
 * [433] Minimum Genetic Mutation
 *
 * A gene string can be represented by an 8-character long string, with choices from 'A', 'C', 'G', and 'T'.
 * Suppose we need to investigate a mutation from a gene string startGene to a gene string endGene where one mutation is defined as one single character changed in the gene string.
 * 
 * 	For example, "AACCGGTT" --> "AACCGGTA" is one mutation.
 * 
 * There is also a gene bank bank that records all the valid gene mutations. A gene must be in bank to make it a valid gene string.
 * Given the two gene strings startGene and endGene and the gene bank bank, return the minimum number of mutations needed to mutate from startGene to endGene. If there is no such a mutation, return -1.
 * Note that the starting point is assumed to be valid, so it might not be included in the bank.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: startGene = "AACCGGTT", endGene = "AACCGGTA", bank = ["AACCGGTA"]
 * Output: 1
 * 
 * <strong class="example">Example 2:
 * 
 * Input: startGene = "AACCGGTT", endGene = "AAACGGTA", bank = ["AACCGGTA","AACCGCTA","AAACGGTA"]
 * Output: 2
 * 
 *  
 * Constraints:
 * 
 * 	0 <= bank.length <= 10
 * 	startGene.length == endGene.length == bank[i].length == 8
 * 	startGene, endGene, and bank[i] consist of only the characters ['A', 'C', 'G', 'T'].
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-genetic-mutation/
// discuss: https://leetcode.com/problems/minimum-genetic-mutation/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_mutation(start_gene: String, end_gene: String, bank: Vec<String>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_433() {
    }
}
