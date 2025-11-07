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
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = std::collections::HashMap::new();
        for (i, &v) in nums.iter().enumerate() {
            if let Some(&j) = map.get(&(target - v)) {
                return vec![j as i32, i as i32];
            }
            map.insert(v, i);
        }
        vec![]
    }
}
//leetcode submit region end(Prohibit modification and deletion)
