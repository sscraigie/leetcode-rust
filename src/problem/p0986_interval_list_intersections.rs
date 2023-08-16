/**
 * [986] Interval List Intersections
 *
 * You are given two lists of closed intervals, firstList and secondList, where firstList[i] = [starti, endi] and secondList[j] = [startj, endj]. Each list of intervals is pairwise disjoint and in sorted order.
 * Return the intersection of these two interval lists.
 * A closed interval [a, b] (with a <= b) denotes the set of real numbers x with a <= x <= b.
 * The intersection of two closed intervals is a set of real numbers that are either empty or represented as a closed interval. For example, the intersection of [1, 3] and [2, 4] is [2, 3].
 *  
 * <strong class="example">Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2019/01/30/interval1.png" style="width: 700px; height: 194px;" />
 * Input: firstList = [[0,2],[5,10],[13,23],[24,25]], secondList = [[1,5],[8,12],[15,24],[25,26]]
 * Output: [[1,2],[5,5],[8,10],[15,23],[24,24],[25,25]]
 * 
 * <strong class="example">Example 2:
 * 
 * Input: firstList = [[1,3],[5,9]], secondList = []
 * Output: []
 * 
 *  
 * Constraints:
 * 
 * 	0 <= firstList.length, secondList.length <= 1000
 * 	firstList.length + secondList.length >= 1
 * 	0 <= starti < endi <= 10^9
 * 	endi < starti+1
 * 	0 <= startj < endj <= 10^9 
 * 	endj < startj+1
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/interval-list-intersections/
// discuss: https://leetcode.com/problems/interval-list-intersections/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn interval_intersection(first_list: Vec<Vec<i32>>, second_list: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_986() {
    }
}
