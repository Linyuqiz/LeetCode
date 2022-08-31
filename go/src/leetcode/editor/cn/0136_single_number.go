package main

func main() {
	println("136、single_number")
}

// leetcode submit region begin(Prohibit modification and deletion)
func singleNumber(nums []int) int {
	number := 0
	for _, num := range nums {
		number ^= num
	}
	return number
}

//leetcode submit region end(Prohibit modification and deletion)
