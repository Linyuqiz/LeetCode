package main

func main() {
	println("268、missing_number")
}

// leetcode submit region begin(Prohibit modification and deletion)
func missingNumber(nums []int) int {
	var res int
	for i, v := range nums {
		res ^= i ^ v
	}
	return res ^ len(nums)
}

//leetcode submit region end(Prohibit modification and deletion)
