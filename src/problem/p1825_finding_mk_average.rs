/**
 * [1825] Finding MK Average
 *
 * You are given two integers, m and k, and a stream of integers. You are tasked to implement a data structure that calculates the MKAverage for the stream.
 * The MKAverage can be calculated using these steps:
 * <ol>
 * 	If the number of the elements in the stream is less than m you should consider the MKAverage to be -1. Otherwise, copy the last m elements of the stream to a separate container.
 * 	Remove the smallest k elements and the largest k elements from the container.
 * 	Calculate the average value for the rest of the elements rounded down to the nearest integer.
 * </ol>
 * Implement the MKAverage class:
 * 
 * 	MKAverage(int m, int k) Initializes the MKAverage object with an empty stream and the two integers m and k.
 * 	void addElement(int num) Inserts a new element num into the stream.
 * 	int calculateMKAverage() Calculates and returns the MKAverage for the current stream rounded down to the nearest integer.
 * 
 *  
 * <strong class="example">Example 1:
 * 
 * Input
 * ["MKAverage", "addElement", "addElement", "calculateMKAverage", "addElement", "calculateMKAverage", "addElement", "addElement", "addElement", "calculateMKAverage"]
 * [[3, 1], [3], [1], [], [10], [], [5], [5], [5], []]
 * Output
 * [null, null, null, -1, null, 3, null, null, null, 5]
 * Explanation
 * MKAverage obj = new MKAverage(3, 1); 
 * obj.addElement(3);        // current elements are [3]
 * obj.addElement(1);        // current elements are [3,1]
 * obj.calculateMKAverage(); // return -1, because m = 3 and only 2 elements exist.
 * obj.addElement(10);       // current elements are [3,1,10]
 * obj.calculateMKAverage(); // The last 3 elements are [3,1,10].
 *                           // After removing smallest and largest 1 element the container will be [3].
 *                           // The average of [3] equals 3/1 = 3, return 3
 * obj.addElement(5);        // current elements are [3,1,10,5]
 * obj.addElement(5);        // current elements are [3,1,10,5,5]
 * obj.addElement(5);        // current elements are [3,1,10,5,5,5]
 * obj.calculateMKAverage(); // The last 3 elements are [5,5,5].
 *                           // After removing smallest and largest 1 element the container will be [5].
 *                           // The average of [5] equals 5/1 = 5, return 5
 * 
 *  
 * Constraints:
 * 
 * 	3 <= m <= 10^5
 * 	1 <= k*2 < m
 * 	1 <= num <= 10^5
 * 	At most 10^5 calls will be made to addElement and calculateMKAverage.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/finding-mk-average/
// discuss: https://leetcode.com/problems/finding-mk-average/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

struct MKAverage {
        false
    }


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MKAverage {

    fn new(m: i32, k: i32) -> Self {
        
    }
    
    fn add_element(&self, num: i32) {
        
    }
    
    fn calculate_mk_average(&self) -> i32 {
        
    }
}

/**
 * Your MKAverage object will be instantiated and called as such:
 * let obj = MKAverage::new(m, k);
 * obj.add_element(num);
 * let ret_2: i32 = obj.calculate_mk_average();
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1825() {
    }
}
