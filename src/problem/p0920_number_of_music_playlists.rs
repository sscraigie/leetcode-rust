/**
 * [920] Number of Music Playlists
 *
 * Your music player contains n different songs. You want to listen to goal songs (not necessarily different) during your trip. To avoid boredom, you will create a playlist so that:
 * 
 * 	Every song is played at least once.
 * 	A song can only be played again only if k other songs have been played.
 * 
 * Given n, goal, and k, return the number of possible playlists that you can create. Since the answer can be very large, return it modulo 10^9 + 7.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: n = 3, goal = 3, k = 1
 * Output: 6
 * Explanation: There are 6 possible playlists: [1, 2, 3], [1, 3, 2], [2, 1, 3], [2, 3, 1], [3, 1, 2], and [3, 2, 1].
 * 
 * <strong class="example">Example 2:
 * 
 * Input: n = 2, goal = 3, k = 0
 * Output: 6
 * Explanation: There are 6 possible playlists: [1, 1, 2], [1, 2, 1], [2, 1, 1], [2, 2, 1], [2, 1, 2], and [1, 2, 2].
 * 
 * <strong class="example">Example 3:
 * 
 * Input: n = 2, goal = 3, k = 1
 * Output: 2
 * Explanation: There are 2 possible playlists: [1, 2, 1] and [2, 1, 2].
 * 
 *  
 * Constraints:
 * 
 * 	0 <= k < n <= goal <= 100
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/number-of-music-playlists/
// discuss: https://leetcode.com/problems/number-of-music-playlists/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn num_music_playlists(n: i32, goal: i32, k: i32) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_920() {
    }
}
