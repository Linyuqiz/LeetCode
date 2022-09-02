#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test() {
        println!("219、contains_duplicate_ii");
    }
}

#[allow(dead_code)]
struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    #[allow(dead_code)]
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut hash = std::collections::HashMap::new();
        for (i, v) in nums.iter().enumerate() {
            if let Some(value) = hash.insert(v, i) {
                if i - value <= k as usize {
                    return true;
                }
            }
        }
        false
    }
}
//leetcode submit region end(Prohibit modification and deletion)
