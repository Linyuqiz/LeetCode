package main

func main() {
	println("463、island_perimeter")
}

// leetcode submit region begin(Prohibit modification and deletion)
func islandPerimeter(grid [][]int) int {
	type pair struct{ x, y int }
	pairs, m, n, res := []pair{{-1, 0}, {1, 0}, {0, -1}, {0, 1}}, len(grid), len(grid[0]), 0
	var markGrid func(x, y int)
	markGrid = func(x, y int) {
		if x < 0 || x >= m || y < 0 || y >= n || grid[x][y] == 0 {
			res++
			return
		}
		if grid[x][y] == 2 {
			return
		}
		grid[x][y] = 2
		for _, p := range pairs {
			markGrid(x+p.x, y+p.y)
		}
	}
	for i, row := range grid {
		for j, v := range row {
			if v == 1 {
				markGrid(i, j)
			}
		}
	}
	return res
}

//leetcode submit region end(Prohibit modification and deletion)
