package main

func main() {
	println("119、pascals_triangle_ii")
}

// leetcode submit region begin(Prohibit modification and deletion)
func getRow(rowIndex int) []int {
	// 1  3  3  1
	// 1  4  6  4  1
	row := make([]int, rowIndex+1)
	row[0] = 1
	for i := 1; i <= rowIndex; i++ {
		for j := i; j > 0; j-- {
			row[j] += row[j-1]
		}
	}
	return row
}

//leetcode submit region end(Prohibit modification and deletion)
