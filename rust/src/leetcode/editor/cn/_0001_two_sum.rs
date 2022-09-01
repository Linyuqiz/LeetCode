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
        let mut hash = std::collections::HashMap::<i32, i32>::new();
        for (i, v) in nums.iter().enumerate() {
            // 注意：fn 的入参是值还是值的引用
            let value = target - v;
            if hash.contains_key(&value) {
                return vec![*hash.get(&value).unwrap(), i as i32];
            }
            // 这是个坑！没有加结尾的分号：hash.insert(*v, i as i32)
            hash.insert(*v, i as i32);
        }
        return vec![];
    }
}
//leetcode submit region end(Prohibit modification and deletion)

