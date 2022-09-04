#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test() {
        println!("163、missing_ranges");
    }
}

#[allow(dead_code)]
struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    #[allow(dead_code)]
    pub fn find_missing_ranges(mut nums: Vec<i32>, lower: i32, upper: i32) -> Vec<String> {
        nums.insert(0, lower - 1);
        nums.push(upper + 1);
        let mut res = vec![];
        for i in 1..nums.len() {
            let value = nums[i] - nums[i - 1];
            if value == 2 {
                res.push((nums[i] - 1).to_string())
            } else if value > 2 {
                res.push(
                    (nums[i - 1] + 1).to_string() + &*"->".to_owned() + &*(nums[i] - 1).to_string(),
                )
            }
        }
        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
