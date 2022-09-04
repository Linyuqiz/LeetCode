#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test() {
        println!("121、best_time_to_buy_and_sell_stock");
    }
}

#[allow(dead_code)]
struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    #[allow(dead_code)]
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut res, mut min) = (0, prices[0]);
        for i in 0..prices.len() {
            if prices[i] < min {
                min = prices[i];
            } else if prices[i] - min > res {
                res = prices[i] - min;
            }
        }
        res
    }
}
//leetcode submit region end(Prohibit modification and deletion)
