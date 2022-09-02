#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test() {
        println!("349、intersection_of_two_arrays");
    }
}

#[allow(dead_code)]
struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    #[allow(dead_code)]
    pub fn intersection(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
        nums1.sort_unstable();
        nums2.sort_unstable();
        let (mut left, mut right, mut res) = (0, 0, vec![]);
        while left < nums1.len() && right < nums2.len() {
            let (x, y) = (nums1[left], nums2[right]);
            if x == y {
                if res.len() == 0 || x > res[res.len() - 1] {
                    res.insert(res.len(), x);
                }
                left += 1;
                right += 1;
            } else if x < y {
                left += 1;
            } else {
                right += 1;
            }
        }
        return res;
    }

    #[allow(dead_code)]
    pub fn intersection0(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        nums1
            .into_iter()
            .collect::<std::collections::HashSet<i32>>()
            .intersection(
                &nums2
                    .into_iter()
                    .collect::<std::collections::HashSet<i32>>(),
            )
            .map(|&x| x)
            .collect()
    }
}
//leetcode submit region end(Prohibit modification and deletion)
