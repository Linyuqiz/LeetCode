#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test() {
        println!("119、pascals_triangle_ii");
    }
}

#[allow(dead_code)]
struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    #[allow(dead_code)]
    pub fn get_row(row_index: i32) -> Vec<i32> {
        // 1  3  3  1
        // 1  4  6  4  1
        let mut res = vec![];
        for i in 0..=row_index as usize {
            res.push(1);
            for j in (1..i).rev() {
                res[j] = res[j] + res[j - 1]
            }
        }
        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
