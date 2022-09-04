#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test() {
        println!("118、pascals_triangle");
    }
}

#[allow(dead_code)]
struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    #[allow(dead_code)]
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        for i in 0..num_rows as usize {
            res.push(vec![1; i + 1]);
            for j in 1..i {
                res[i][j] = res[i - 1][j] + res[i - 1][j - 1]
            }
        }
        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
