/**
 * [2659] Make Array Empty
 *
 * You are given an integer array nums containing distinct numbers, and you can perform the following operations until the array is empty:
 * 
 * 	If the first element has the smallest value, remove it
 * 	Otherwise, put the first element at the end of the array.
 * 
 * Return an integer denoting the number of operations it takes to make nums empty.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: nums = [3,4,-1]
 * Output: 5
 * 
 * <table style="border: 2px solid black; border-collapse: collapse;">
 * 	<thead>
 * 		<tr>
 * 			<th style="border: 2px solid black; padding: 5px;">Operation</th>
 * 			<th style="border: 2px solid black; padding: 5px;">Array</th>
 * 		</tr>
 * 	</thead>
 * 	<tbody>
 * 		<tr>
 * 			<td style="border: 2px solid black; padding: 5px;">1</td>
 * 			<td style="border: 2px solid black; padding: 5px;">[4, -1, 3]</td>
 * 		</tr>
 * 		<tr>
 * 			<td style="border: 2px solid black; padding: 5px;">2</td>
 * 			<td style="border: 2px solid black; padding: 5px;">[-1, 3, 4]</td>
 * 		</tr>
 * 		<tr>
 * 			<td style="border: 2px solid black; padding: 5px;">3</td>
 * 			<td style="border: 2px solid black; padding: 5px;">[3, 4]</td>
 * 		</tr>
 * 		<tr>
 * 			<td style="border: 2px solid black; padding: 5px;">4</td>
 * 			<td style="border: 2px solid black; padding: 5px;">[4]</td>
 * 		</tr>
 * 		<tr>
 * 			<td style="border: 2px solid black; padding: 5px;">5</td>
 * 			<td style="border: 2px solid black; padding: 5px;">[]</td>
 * 		</tr>
 * 	</tbody>
 * </table>
 * <strong class="example">Example 2:
 * 
 * Input: nums = [1,2,4,3]
 * Output: 5
 * 
 * <table style="border: 2px solid black; border-collapse: collapse;">
 * 	<thead>
 * 		<tr>
 * 			<th style="border: 2px solid black; padding: 5px;">Operation</th>
 * 			<th style="border: 2px solid black; padding: 5px;">Array</th>
 * 		</tr>
 * 	</thead>
 * 	<tbody>
 * 		<tr>
 * 			<td style="border: 2px solid black; padding: 5px;">1</td>
 * 			<td style="border: 2px solid black; padding: 5px;">[2, 4, 3]</td>
 * 		</tr>
 * 		<tr>
 * 			<td style="border: 2px solid black; padding: 5px;">2</td>
 * 			<td style="border: 2px solid black; padding: 5px;">[4, 3]</td>
 * 		</tr>
 * 		<tr>
 * 			<td style="border: 2px solid black; padding: 5px;">3</td>
 * 			<td style="border: 2px solid black; padding: 5px;">[3, 4]</td>
 * 		</tr>
 * 		<tr>
 * 			<td style="border: 2px solid black; padding: 5px;">4</td>
 * 			<td style="border: 2px solid black; padding: 5px;">[4]</td>
 * 		</tr>
 * 		<tr>
 * 			<td style="border: 2px solid black; padding: 5px;">5</td>
 * 			<td style="border: 2px solid black; padding: 5px;">[]</td>
 * 		</tr>
 * 	</tbody>
 * </table>
 * <strong class="example">Example 3:
 * 
 * Input: nums = [1,2,3]
 * Output: 3
 * 
 * <table style="border: 2px solid black; border-collapse: collapse;">
 * 	<thead>
 * 		<tr>
 * 			<th style="border: 2px solid black; padding: 5px;">Operation</th>
 * 			<th style="border: 2px solid black; padding: 5px;">Array</th>
 * 		</tr>
 * 	</thead>
 * 	<tbody>
 * 		<tr>
 * 			<td style="border: 2px solid black; padding: 5px;">1</td>
 * 			<td style="border: 2px solid black; padding: 5px;">[2, 3]</td>
 * 		</tr>
 * 		<tr>
 * 			<td style="border: 2px solid black; padding: 5px;">2</td>
 * 			<td style="border: 2px solid black; padding: 5px;">[3]</td>
 * 		</tr>
 * 		<tr>
 * 			<td style="border: 2px solid black; padding: 5px;">3</td>
 * 			<td style="border: 2px solid black; padding: 5px;">[]</td>
 * 		</tr>
 * 	</tbody>
 * </table>
 *  
 * Constraints:
 * 
 * 	1 <= nums.length <= 10^5
 * 	-10^9 <= nums[i] <= 10^9
 * 	All values in nums are distinct.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/make-array-empty/
// discuss: https://leetcode.com/problems/make-array-empty/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_operations_to_empty_array(nums: Vec<i32>) -> i64 {
        
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2659() {
    }
}
