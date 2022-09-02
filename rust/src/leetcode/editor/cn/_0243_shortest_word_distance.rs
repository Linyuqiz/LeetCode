#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test() {
        println!("243、shortest_word_distance");
    }
}

#[allow(dead_code)]
struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    #[allow(dead_code)]
    pub fn shortest_distance(words_dict: Vec<String>, word1: String, word2: String) -> i32 {
        let (mut res, mut left, mut right) = (words_dict.len() as i32, -1, -1);
        for i in 0..words_dict.len() {
            if word1 == words_dict[i as usize] {
                left = i as i32;
            }
            if word2 == words_dict[i as usize] {
                right = i as i32;
            }
            if left > -1 && right > -1 {
                res = ((right - left) as i32).abs().min(res)
            }
        }
        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
