/**
 * [710] Random Pick with Blacklist
 *
 * You are given an integer n and an array of unique integers blacklist. Design an algorithm to pick a random integer in the range [0, n - 1] that is not in blacklist. Any integer that is in the mentioned range and not in blacklist should be equally likely to be returned.
 * Optimize your algorithm such that it minimizes the number of calls to the built-in random function of your language.
 * Implement the Solution class:
 * 
 * 	Solution(int n, int[] blacklist) Initializes the object with the integer n and the blacklisted integers blacklist.
 * 	int pick() Returns a random integer in the range [0, n - 1] and not in blacklist.
 * 
 *  
 * <strong class="example">Example 1:
 * 
 * Input
 * ["Solution", "pick", "pick", "pick", "pick", "pick", "pick", "pick"]
 * [[7, [2, 3, 5]], [], [], [], [], [], [], []]
 * Output
 * [null, 0, 4, 1, 6, 1, 0, 4]
 * Explanation
 * Solution solution = new Solution(7, [2, 3, 5]);
 * solution.pick(); // return 0, any integer from [0,1,4,6] should be ok. Note that for every call of pick,
 *                  // 0, 1, 4, and 6 must be equally likely to be returned (i.e., with probability 1/4).
 * solution.pick(); // return 4
 * solution.pick(); // return 1
 * solution.pick(); // return 6
 * solution.pick(); // return 1
 * solution.pick(); // return 0
 * solution.pick(); // return 4
 * 
 *  
 * Constraints:
 * 
 * 	1 <= n <= 10^9
 * 	0 <= blacklist.length <= min(10^5, n - 1)
 * 	0 <= blacklist[i] < n
 * 	All the values of blacklist are unique.
 * 	At most 2 * 10^4 calls will be made to pick.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/random-pick-with-blacklist/
// discuss: https://leetcode.com/problems/random-pick-with-blacklist/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

struct Solution {
        vec![]
    }


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {

    fn new(n: i32, blacklist: Vec<i32>) -> Self {
        
    }
    
    fn pick(&self) -> i32 {
        
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(n, blacklist);
 * let ret_1: i32 = obj.pick();
 */

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_710() {
    }
}
