/**
 * [432] All O`one Data Structure
 *
 * Design a data structure to store the strings' count with the ability to return the strings with minimum and maximum counts.
 * Implement the AllOne class:
 * 
 * 	AllOne() Initializes the object of the data structure.
 * 	inc(String key) Increments the count of the string key by 1. If key does not exist in the data structure, insert it with count 1.
 * 	dec(String key) Decrements the count of the string key by 1. If the count of key is 0 after the decrement, remove it from the data structure. It is guaranteed that key exists in the data structure before the decrement.
 * 	getMaxKey() Returns one of the keys with the maximal count. If no element exists, return an empty string "".
 * 	getMinKey() Returns one of the keys with the minimum count. If no element exists, return an empty string "".
 * 
 * Note that each function must run in O(1) average time complexity.
 *  
 * <strong class="example">Example 1:
 * 
 * Input
 * ["AllOne", "inc", "inc", "getMaxKey", "getMinKey", "inc", "getMaxKey", "getMinKey"]
 * [[], ["hello"], ["hello"], [], [], ["leet"], [], []]
 * Output
 * [null, null, null, "hello", "hello", null, "hello", "leet"]
 * Explanation
 * AllOne allOne = new AllOne();
 * allOne.inc("hello");
 * allOne.inc("hello");
 * allOne.getMaxKey(); // return "hello"
 * allOne.getMinKey(); // return "hello"
 * allOne.inc("leet");
 * allOne.getMaxKey(); // return "hello"
 * allOne.getMinKey(); // return "leet"
 * 
 *  
 * Constraints:
 * 
 * 	1 <= key.length <= 10
 * 	key consists of lowercase English letters.
 * 	It is guaranteed that for each call to dec, key is existing in the data structure.
 * 	At most 5 * 10^4 calls will be made to inc, dec, getMaxKey, and getMinKey.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/all-oone-data-structure/
// discuss: https://leetcode.com/problems/all-oone-data-structure/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

struct AllOne {
        false
    }


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl AllOne {

    fn new() -> Self {
        
    }
    
    fn inc(&self, key: String) {
        
    }
    
    fn dec(&self, key: String) {
        
    }
    
    fn get_max_key(&self) -> String {
        
    }
    
    fn get_min_key(&self) -> String {
        
    }
}

/**
 * Your AllOne object will be instantiated and called as such:
 * let obj = AllOne::new();
 * obj.inc(key);
 * obj.dec(key);
 * let ret_3: String = obj.get_max_key();
 * let ret_4: String = obj.get_min_key();
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_432() {
    }
}
