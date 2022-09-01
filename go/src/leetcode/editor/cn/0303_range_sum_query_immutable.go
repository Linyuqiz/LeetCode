package main

func main() {
	println("303、range_sum_query_immutable")
}

// leetcode submit region begin(Prohibit modification and deletion)
type NumArray struct {
	Data []int
}

func Constructor(nums []int) NumArray {
	nums = append([]int{0}, nums...)
	for i := 1; i < len(nums); i++ {
		nums[i] = nums[i] + nums[i-1]
	}
	return NumArray{
		Data: nums,
	}
}

func (this *NumArray) SumRange(left int, right int) int {
	return this.Data[right+1] - this.Data[left]
}

/**
 * Your NumArray object will be instantiated and called as such:
 * obj := Constructor(nums);
 * param_1 := obj.SumRange(left,right);
 */
//leetcode submit region end(Prohibit modification and deletion)
