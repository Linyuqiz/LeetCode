#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test() {
        println!("1、two_sum");
    }
}

#[allow(dead_code)]
struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    #[allow(dead_code)]
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut number_map = std::collections::HashMap::new();

        for (i, v) in nums.iter().enumerate() {
            if let Some(j) = number_map.get(&(target - v)) {
                return vec![*j, i as i32];
            }
            number_map.insert(v, i as i32);
        }

        vec![]
    }
}
//leetcode submit region end(Prohibit modification and deletion)
