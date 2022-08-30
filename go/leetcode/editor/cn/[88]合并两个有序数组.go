package main

import "sort"

// leetcode submit region begin(Prohibit modification and deletion)
func merge(nums1 []int, m int, nums2 []int, n int) {
	for range nums2 {
		nums1[m] = nums2[n-1]
		m++
		n--
	}
	sort.Ints(nums1)
}

//leetcode submit region end(Prohibit modification and deletion)
