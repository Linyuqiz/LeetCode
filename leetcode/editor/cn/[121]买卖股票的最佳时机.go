package main

// leetcode submit region begin(Prohibit modification and deletion)
func maxProfit(prices []int) int {
	res, minValue := 0, prices[0]
	for i := 0; i < len(prices); i++ {
		if prices[i] < minValue {
			minValue = prices[i]
		} else if prices[i]-minValue > res {
			res = prices[i] - minValue
		}
	}
	return res
}

//leetcode submit region end(Prohibit modification and deletion)
