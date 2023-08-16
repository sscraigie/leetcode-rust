/**
 * [857] Minimum Cost to Hire K Workers
 *
 * There are n workers. You are given two integer arrays quality and wage where quality[i] is the quality of the i^th worker and wage[i] is the minimum wage expectation for the i^th worker.
 * We want to hire exactly k workers to form a paid group. To hire a group of k workers, we must pay them according to the following rules:
 * <ol>
 * 	Every worker in the paid group should be paid in the ratio of their quality compared to other workers in the paid group.
 * 	Every worker in the paid group must be paid at least their minimum wage expectation.
 * </ol>
 * Given the integer k, return the least amount of money needed to form a paid group satisfying the above conditions. Answers within 10^-5 of the actual answer will be accepted.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: quality = [10,20,5], wage = [70,50,30], k = 2
 * Output: 105.00000
 * Explanation: We pay 70 to 0^th worker and 35 to 2^nd worker.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: quality = [3,1,10,10,1], wage = [4,8,2,2,7], k = 3
 * Output: 30.66667
 * Explanation: We pay 4 to 0^th worker, 13.33333 to 2^nd and 3^rd workers separately.
 * 
 *  
 * Constraints:
 * 
 * 	n == quality.length == wage.length
 * 	1 <= k <= n <= 10^4
 * 	1 <= quality[i], wage[i] <= 10^4
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-cost-to-hire-k-workers/
// discuss: https://leetcode.com/problems/minimum-cost-to-hire-k-workers/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
        0f64
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_857() {
    }
}
