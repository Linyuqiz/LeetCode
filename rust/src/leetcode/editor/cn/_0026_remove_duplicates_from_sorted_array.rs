#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test() {
        println!("26、remove_duplicates_from_sorted_array");
    }
}

#[allow(dead_code)]
struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    #[allow(dead_code)]
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        nums.dedup(); // 去除相邻间的重复元素
        nums.len() as i32
    }
}
//leetcode submit region end(Prohibit modification and deletion)

