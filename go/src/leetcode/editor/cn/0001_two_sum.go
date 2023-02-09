package main

func main() {
	println("1、two_sum")
}

// leetcode submit region begin(Prohibit modification and deletion)
func twoSum(nums []int, target int) []int {
	m := make(map[int]int)
	for i, v := range nums {
		t := target - v
		if r, o := m[v]; o {
			return []int{r, i}
		}
		m[t] = i
	}
	return []int{}
}

//leetcode submit region end(Prohibit modification and deletion)
