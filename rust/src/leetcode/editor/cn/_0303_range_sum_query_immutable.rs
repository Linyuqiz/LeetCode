#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test() {
        println!("303、range_sum_query_immutable");
    }
}

#[allow(dead_code)]
struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
#[allow(dead_code)]
struct NumArray {
    pub data: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    #[allow(dead_code)]
    fn new(mut nums: Vec<i32>) -> Self {
        nums.insert(0, 0);
        for i in 1..nums.len() {
            nums[i] = nums[i] + nums[i - 1];
        }
        NumArray { data: nums }
    }

    #[allow(dead_code)]
    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.data[right as usize + 1] - self.data[left as usize]
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * let ret_1: i32 = obj.sum_range(left, right);
 */
#[allow(dead_code)]
const PLACEHOLDER: &str = "";
//leetcode submit region end(Prohibit modification and deletion)
