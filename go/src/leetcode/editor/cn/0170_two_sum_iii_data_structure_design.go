package main

func main() {
	println("170、two_sum_iii_data_structure_design")
}

// leetcode submit region begin(Prohibit modification and deletion)
type TwoSum struct {
	L []int
	M map[int]int
}

func Constructor() TwoSum {
	return TwoSum{
		L: make([]int, 0),
		M: make(map[int]int),
	}
}

func (this *TwoSum) Add(number int) {
	this.L = append(this.L, number)
	this.M[number] = len(this.L) - 1
}

func (this *TwoSum) Find(value int) bool {
	for i, v := range this.L {
		t := value - v
		if j, o := this.M[t]; o && i != j {
			return true
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
