package main

import (
	"sort"
	"strconv"
)

// leetcode submit region begin(Prohibit modification and deletion)
func findMissingRanges(nums []int, lower int, upper int) []string {
	res := make([]string, 0)
	nums = append(nums, lower, upper)
	sort.Ints(nums)
	for i := 1; i < len(nums); i++ {
		value := nums[i] - nums[i-1]
		if value == 2 {
			res = append(res, strconv.Itoa(nums[i]-1))
		} else if value > 2 {
			res = append(res, strconv.Itoa(nums[i-1]+1)+"->"+strconv.Itoa(nums[i]-1))
		}
	}
	return res
}

//leetcode submit region end(Prohibit modification and deletion)
