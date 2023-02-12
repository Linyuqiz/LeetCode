package main

import "math"

func main() {
	println("270、closest_binary_search_tree_value")
}

//leetcode submit region begin(Prohibit modification and deletion)
/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func closestValue(root *TreeNode, target float64) int {
	c, r := root, root.Val
	for c != nil {
		if target == float64(c.Val) {
			return c.Val
		}
		if math.Abs(target-float64(c.Val)) < math.Abs(target-float64(r)) {
			r = c.Val
		}
		if target < float64(c.Val) {
			c = c.Left
		} else {
			c = c.Right
		}
	}
	return r
}

//leetcode submit region end(Prohibit modification and deletion)

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}
