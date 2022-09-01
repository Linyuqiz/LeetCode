package main

import "sort"

func main() {
	println("414、third_maximum_number")
}

// leetcode submit region begin(Prohibit modification and deletion)
func thirdMax(nums []int) int {
	sort.Ints(nums)
	res, count := 0, 0
	for i := len(nums) - 1; i >= 0; i-- {
		if i == len(nums)-1 || nums[i] != nums[i+1] {
			res = nums[i]
			count++
		}
		if count == 3 {
			return res
		}
	}
	return nums[len(nums)-1]
}

//leetcode submit region end(Prohibit modification and deletion)
