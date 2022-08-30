package main

import "sort"

func main() {
	println("170、two_sum_iii_data_structure_design")
}

// leetcode submit region begin(Prohibit modification and deletion)
type TwoSum struct {
	data   []int
	sorted bool
}

func Constructor() TwoSum {
	return TwoSum{
		data: make([]int, 0),
	}
}

func (this *TwoSum) Add(number int) {
	this.data = append(this.data, number)
	this.sorted = false
}

func (this *TwoSum) Find(value int) bool {
	if !this.sorted {
		sort.Ints(this.data)
		this.sorted = true
	}
	left, right := 0, len(this.data)-1
	for left < right {
		if this.data[left]+this.data[right] == value {
			return true
		} else if this.data[left]+this.data[right] < value {
			left++
		} else if this.data[left]+this.data[right] > value {
			right--
		}
	}
	return false
}

/**
 * Your TwoSum object will be instantiated and called as such:
 * obj := Constructor();
 * obj.Add(number);
 * param_2 := obj.Find(value);
 */
//leetcode submit region end(Prohibit modification and deletion)
