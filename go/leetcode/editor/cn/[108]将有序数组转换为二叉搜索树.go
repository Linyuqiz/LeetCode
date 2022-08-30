package main

import (
	"math/rand"
	"time"
)

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
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
func sortedArrayToBST(nums []int) *TreeNode {
	rand.Seed(time.Now().UnixNano())
	return getArrayToBST(nums, 0, len(nums)-1)
}

func getArrayToBST(nums []int, left, right int) *TreeNode {
	if left > right {
		return nil
	}
	//mid := (left + right) / 2
	//mid := (left + right + 1) / 2
	mid := (left + right + rand.Intn(2)) / 2
	root := &TreeNode{
		Val: nums[mid],
	}
	root.Left = getArrayToBST(nums, left, mid-1)
	root.Right = getArrayToBST(nums, mid+1, right)
	return root
}

//leetcode submit region end(Prohibit modification and deletion)
