#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test() {
        println!("88、merge_sorted_array");
    }
}

#[allow(dead_code)]
struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    #[allow(dead_code)]
    pub fn merge(nums1: &mut Vec<i32>, mut m: i32, nums2: &mut Vec<i32>, mut n: i32) {
        for _ in 0..nums2.len() {
            nums1[m as usize] = nums2[(n - 1) as usize];
            m += 1;
            n -= 1;
        }
        nums1.sort_unstable()
    }
}
//leetcode submit region end(Prohibit modification and deletion)
