/**
 * [2276] Count Integers in Intervals
 *
 * Given an empty set of intervals, implement a data structure that can:
 * 
 * 	Add an interval to the set of intervals.
 * 	Count the number of integers that are present in at least one interval.
 * 
 * Implement the CountIntervals class:
 * 
 * 	CountIntervals() Initializes the object with an empty set of intervals.
 * 	void add(int left, int right) Adds the interval [left, right] to the set of intervals.
 * 	int count() Returns the number of integers that are present in at least one interval.
 * 
 * Note that an interval [left, right] denotes all the integers x where left <= x <= right.
 *  
 * <strong class="example">Example 1:
 * 
 * Input
 * ["CountIntervals", "add", "add", "count", "add", "count"]
 * [[], [2, 3], [7, 10], [], [5, 8], []]
 * Output
 * [null, null, null, 6, null, 8]
 * Explanation
 * CountIntervals countIntervals = new CountIntervals(); // initialize the object with an empty set of intervals. 
 * countIntervals.add(2, 3);  // add [2, 3] to the set of intervals.
 * countIntervals.add(7, 10); // add [7, 10] to the set of intervals.
 * countIntervals.count();    // return 6
 *                            // the integers 2 and 3 are present in the interval [2, 3].
 *                            // the integers 7, 8, 9, and 10 are present in the interval [7, 10].
 * countIntervals.add(5, 8);  // add [5, 8] to the set of intervals.
 * countIntervals.count();    // return 8
 *                            // the integers 2 and 3 are present in the interval [2, 3].
 *                            // the integers 5 and 6 are present in the interval [5, 8].
 *                            // the integers 7 and 8 are present in the intervals [5, 8] and [7, 10].
 *                            // the integers 9 and 10 are present in the interval [7, 10].
 * 
 *  
 * Constraints:
 * 
 * 	1 <= left <= right <= 10^9
 * 	At most 10^5 calls in total will be made to add and count.
 * 	At least one call will be made to count.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-integers-in-intervals/
// discuss: https://leetcode.com/problems/count-integers-in-intervals/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

struct CountIntervals {
        false
    }


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CountIntervals {

    fn new() -> Self {
        
    }
    
    fn add(&self, left: i32, right: i32) {
        
    }
    
    fn count(&self) -> i32 {
        
    }
}

/**
 * Your CountIntervals object will be instantiated and called as such:
 * let obj = CountIntervals::new();
 * obj.add(left, right);
 * let ret_2: i32 = obj.count();
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2276() {
    }
}
