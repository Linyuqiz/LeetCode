#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test() {
        println!("27、remove_element");
    }
}

#[allow(dead_code)]
struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    #[allow(dead_code)]
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let (mut left, mut right) = (0, 0);
        while right < nums.len() {
            if nums[right] != val {
                nums[left] = nums[right];
                left += 1;
            }
            right += 1;
        }
        left as i32
    }

    #[allow(dead_code)]
    pub fn remove_element0(nums: &mut Vec<i32>, val: i32) -> i32 {
        nums.retain(|x| *x != val);
        nums.len() as i32
    }
}
//leetcode submit region end(Prohibit modification and deletion)