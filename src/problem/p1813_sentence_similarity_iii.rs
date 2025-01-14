/**
 * [1813] Sentence Similarity III
 *
 * A sentence is a list of words that are separated by a single space with no leading or trailing spaces. For example, "Hello World", "HELLO", "hello world hello world" are all sentences. Words consist of only uppercase and lowercase English letters.
 * Two sentences sentence1 and sentence2 are similar if it is possible to insert an arbitrary sentence (possibly empty) inside one of these sentences such that the two sentences become equal. For example, sentence1 = "Hello my name is Jane" and sentence2 = "Hello Jane" can be made equal by inserting "my name is" between "Hello" and "Jane" in sentence2.
 * Given two sentences sentence1 and sentence2, return true if sentence1 and sentence2 are similar. Otherwise, return false.
 *  
 * <strong class="example">Example 1:
 * 
 * Input: sentence1 = "My name is Haley", sentence2 = "My Haley"
 * Output: true
 * Explanation: sentence2 can be turned to sentence1 by inserting "name is" between "My" and "Haley".
 * 
 * <strong class="example">Example 2:
 * 
 * Input: sentence1 = "of", sentence2 = "A lot of words"
 * Output: false
 * Explanation: No single sentence can be inserted inside one of the sentences to make it equal to the other.
 * 
 * <strong class="example">Example 3:
 * 
 * Input: sentence1 = "Eating right now", sentence2 = "Eating"
 * Output: true
 * Explanation: sentence2 can be turned to sentence1 by inserting "right now" at the end of the sentence.
 * 
 *  
 * Constraints:
 * 
 * 	1 <= sentence1.length, sentence2.length <= 100
 * 	sentence1 and sentence2 consist of lowercase and uppercase English letters and spaces.
 * 	The words in sentence1 and sentence2 are separated by a single space.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/sentence-similarity-iii/
// discuss: https://leetcode.com/problems/sentence-similarity-iii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn are_sentences_similar(sentence1: String, sentence2: String) -> bool {
        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1813() {
    }
}
