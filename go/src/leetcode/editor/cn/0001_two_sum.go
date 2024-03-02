package main

func main() {
	println("1、two_sum")
}

// leetcode submit region begin(Prohibit modification and deletion)
func twoSum(nums []int, target int) []int {
	numberMap := make(map[int]int, 0)

	for i, v := range nums {
		if j, ok := numberMap[target-v]; ok {
			return []int{j, i}
		}
		numberMap[v] = i
	}

	return []int{}
}

//leetcode submit region end(Prohibit modification and deletion)
