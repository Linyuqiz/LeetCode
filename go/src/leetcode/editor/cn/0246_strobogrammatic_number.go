package main

func main() {
	println("246、strobogrammatic_number")
}

// leetcode submit region begin(Prohibit modification and deletion)
func isStrobogrammatic(num string) bool {
	i, mid, s := 0, (len(num)-1)/2, len(num)-1
	m := map[string]string{"0": "0", "1": "1", "8": "8", "6": "9", "9": "6"}
	for i <= mid {
		if v, o := m[string(num[i])]; o && v == string(num[s-i]) {
			i++
		} else {
			return false
		}
	}
	return true
}

//leetcode submit region end(Prohibit modification and deletion)
