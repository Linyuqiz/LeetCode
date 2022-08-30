package main

// leetcode submit region begin(Prohibit modification and deletion)
func containsDuplicate(nums []int) bool {
	hash := make(map[int]bool, 0)
	for _, v := range nums {
		if _, ok := hash[v]; ok {
			return true
		}
		hash[v] = true
	}
	return false
}

//leetcode submit region end(Prohibit modification and deletion)
