package main

func main() {
	println("448、find_all_numbers_disappeared_in_an_array")

	nums := []int{4, 3, 2, 7, 8, 2, 3, 1}
	println(findDisappearedNumbers(nums))
}

// leetcode submit region begin(Prohibit modification and deletion)
func findDisappearedNumbers(nums []int) []int {
	length, res := len(nums), make([]int, 0)
	for _, v := range nums {
		tmp := (v - 1) % length
		nums[tmp] += length
	}
	for i, v := range nums {
		if v <= length {
			res = append(res, i+1)
		}
	}
	return res
}

//leetcode submit region end(Prohibit modification and deletion)
