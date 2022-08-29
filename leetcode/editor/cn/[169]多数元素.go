package main

import "sort"

// leetcode submit region begin(Prohibit modification and deletion)
func majorityElement(nums []int) int {
	sort.Ints(nums)
	return nums[len(nums)/2]
}

//leetcode submit region end(Prohibit modification and deletion)
