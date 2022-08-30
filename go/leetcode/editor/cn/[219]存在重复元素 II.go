package main

// leetcode submit region begin(Prohibit modification and deletion)
func containsNearbyDuplicate(nums []int, k int) bool {
	hash := make(map[int]int, 0)
	for i := 0; i < len(nums); i++ {
		if i > k {
			delete(hash, nums[i-k-1])
		}
		if _, ok := hash[nums[i]]; ok {
			return true
		}
		hash[nums[i]] = i
	}
	return false
}

//leetcode submit region end(Prohibit modification and deletion)
