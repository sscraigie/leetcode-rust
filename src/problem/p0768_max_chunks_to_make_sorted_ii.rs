/**
 * [768] Max Chunks To Make Sorted II
 *
 * You are given an integer array arr.
 * We split arr into some number of chunks (i.e., partitions), and individually sort each chunk. After concatenating them, the result should equal the sorted array.
 * Return the largest number of chunks we can make to sort the array.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: arr = [5,4,3,2,1]
 * Output: 1
 * Explanation:
 * Splitting into two or more chunks will not return the required result.
 * For example, splitting into [5, 4], [3, 2, 1] will result in [4, 5, 1, 2, 3], which isn't sorted.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: arr = [2,1,3,4,4]
 * Output: 4
 * Explanation:
 * We can split into two chunks, such as [2, 1], [3, 4, 4].
 * However, splitting into [2, 1], [3], [4], [4] is the highest number of chunks possible.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= arr.length <= 2000
 * 	0 <= arr[i] <= 10^8
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/max-chunks-to-make-sorted-ii/
// discuss: https://leetcode.com/problems/max-chunks-to-make-sorted-ii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_768() {
    }
}
