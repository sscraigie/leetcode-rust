/**
 * [1286] Iterator for Combination
 *
 * Design the CombinationIterator class:
 * 
 * 	CombinationIterator(string characters, int combinationLength) Initializes the object with a string characters of sorted distinct lowercase English letters and a number combinationLength as arguments.
 * 	next() Returns the next combination of length combinationLength in lexicographical order.
 * 	hasNext() Returns true if and only if there exists a next combination.
 * 
 *  
 * <strong class="example">Example 1:
 * 
 * Input
 * ["CombinationIterator", "next", "hasNext", "next", "hasNext", "next", "hasNext"]
 * [["abc", 2], [], [], [], [], [], []]
 * Output
 * [null, "ab", true, "ac", true, "bc", false]
 * Explanation
 * CombinationIterator itr = new CombinationIterator("abc", 2);
 * itr.next();    // return "ab"
 * itr.hasNext(); // return True
 * itr.next();    // return "ac"
 * itr.hasNext(); // return True
 * itr.next();    // return "bc"
 * itr.hasNext(); // return False
 * 
 *  
 * Constraints:
 * 
 * 	1 <= combinationLength <= characters.length <= 15
 * 	All the characters of characters are unique.
 * 	At most 10^4 calls will be made to next and hasNext.
 * 	It is guaranteed that all calls of the function next are valid.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/iterator-for-combination/
// discuss: https://leetcode.com/problems/iterator-for-combination/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

struct CombinationIterator {
        vec![]
    }


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CombinationIterator {

    fn new(characters: String, combinationLength: i32) -> Self {
        
    }
    
    fn next(&self) -> String {
        
    }
    
    fn has_next(&self) -> bool {
        
    }
}

/**
 * Your CombinationIterator object will be instantiated and called as such:
 * let obj = CombinationIterator::new(characters, combinationLength);
 * let ret_1: String = obj.next();
 * let ret_2: bool = obj.has_next();
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1286() {
    }
}
