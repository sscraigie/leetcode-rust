/**
 * [1233] Remove Sub-Folders from the Filesystem
 *
 * Given a list of folders folder, return the folders after removing all sub-folders in those folders. You may return the answer in any order.
 * If a folder[i] is located within another folder[j], it is called a sub-folder of it.
 * The format of a path is one or more concatenated strings of the form: '/' followed by one or more lowercase English letters.
 * 
 * 	For example, "/leetcode" and "/leetcode/problems" are valid paths while an empty string and "/" are not.
 * 
 *  
 * <strong class="example">Example 1:
 * 
 * Input: folder = ["/a","/a/b","/c/d","/c/d/e","/c/f"]
 * Output: ["/a","/c/d","/c/f"]
 * Explanation: Folders "/a/b" is a subfolder of "/a" and "/c/d/e" is inside of folder "/c/d" in our filesystem.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: folder = ["/a","/a/b/c","/a/b/d"]
 * Output: ["/a"]
 * Explanation: Folders "/a/b/c" and "/a/b/d" will be removed because they are subfolders of "/a".
 * 
 * <strong class="example">Example 3:
 * 
 * Input: folder = ["/a/b/c","/a/b/ca","/a/b/d"]
 * Output: ["/a/b/c","/a/b/ca","/a/b/d"]
 * 
 *  
 * Constraints:
 * 
 * 	1 <= folder.length <= 4 * 10^4
 * 	2 <= folder[i].length <= 100
 * 	folder[i] contains only lowercase letters and '/'.
 * 	folder[i] always starts with the character '/'.
 * 	Each folder name is unique.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/remove-sub-folders-from-the-filesystem/
// discuss: https://leetcode.com/problems/remove-sub-folders-from-the-filesystem/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn remove_subfolders(folder: Vec<String>) -> Vec<String> {
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1233() {
    }
}
