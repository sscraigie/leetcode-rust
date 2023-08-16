/**
 * [2736] Maximum Sum Queries
 *
 * You are given two 0-indexed integer arrays nums1 and nums2, each of length n, and a 1-indexed 2D array queries where queries[i] = [xi, yi].
 * For the i^th query, find the maximum value of nums1[j] + nums2[j] among all indices j (0 <= j < n), where nums1[j] >= xi and nums2[j] >= yi, or -1 if there is no j satisfying the constraints.
 * Return an array answer where answer[i] is the answer to the i^th query.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: nums1 = [4,3,1,2], nums2 = [2,4,9,5], queries = [[4,1],[1,3],[2,5]]
 * Output: [6,10,7]
 * Explanation: 
 * For the 1st query <code node="[object Object]">xi = 4 and <code node="[object Object]">yi = 1, we can select index <code node="[object Object]">j = 0 since <code node="[object Object]">nums1[j] >= 4 and <code node="[object Object]">nums2[j] >= 1. The sum <code node="[object Object]">nums1[j] + nums2[j] is 6, and we can show that 6 is the maximum we can obtain.
 * For the 2nd query <code node="[object Object]">xi = 1 and <code node="[object Object]">yi = 3, we can select index <code node="[object Object]">j = 2 since <code node="[object Object]">nums1[j] >= 1 and <code node="[object Object]">nums2[j] >= 3. The sum <code node="[object Object]">nums1[j] + nums2[j] is 10, and we can show that 10 is the maximum we can obtain. 
 * For the 3rd query <code node="[object Object]">xi = 2 and <code node="[object Object]">yi = 5, we can select index <code node="[object Object]">j = 3 since <code node="[object Object]">nums1[j] >= 2 and <code node="[object Object]">nums2[j] >= 5. The sum <code node="[object Object]">nums1[j] + nums2[j] is 7, and we can show that 7 is the maximum we can obtain.
 * Therefore, we return <code node="[object Object]">[6,10,7].
 * 
 * <strong class="example">Example 2:
 * 
 * Input: nums1 = [3,2,5], nums2 = [2,3,4], queries = [[4,4],[3,2],[1,1]]
 * Output: [9,9,9]
 * Explanation: For this example, we can use index <code node="[object Object]">j = 2 for all the queries since it satisfies the constraints for each query.
 * 
 * <strong class="example">Example 3:
 * 
 * Input: nums1 = [2,1], nums2 = [2,3], queries = [[3,3]]
 * Output: [-1]
 * Explanation: There is one query in this example with <code node="[object Object]">xi = 3 and <code node="[object Object]">yi = 3. For every index, j, either nums1[j] < <code node="[object Object]">xi or nums2[j] < <code node="[object Object]">yi. Hence, there is no solution. 
 * 
 *  
 * Constraints:
 * 
 * 	nums1.length == nums2.length 
 * 	n == nums1.length 
 * 	1 <= n <= 10^5
 * 	1 <= nums1[i], nums2[i] <= 10^9 
 * 	1 <= queries.length <= 10^5
 * 	queries[i].length == 2
 * 	xi == queries[i][1]
 * 	yi == queries[i][2]
 * 	1 <= xi, yi <= 10^9
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-sum-queries/
// discuss: https://leetcode.com/problems/maximum-sum-queries/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn maximum_sum_queries(nums1: Vec<i32>, nums2: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2736() {
    }
}
