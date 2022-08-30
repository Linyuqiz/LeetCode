package main

import "sort"

func main() {
	println("169、majority_element")
}

// leetcode submit region begin(Prohibit modification and deletion)
func majorityElement(nums []int) int {
	sort.Ints(nums)
	return nums[len(nums)/2]
}

//leetcode submit region end(Prohibit modification and deletion)
