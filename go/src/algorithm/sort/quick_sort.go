package main

func QuickSort(arr []int, left, right int) {
	if left < right {
		pivot := getPartition(arr, left, right)
		QuickSort(arr, left, pivot-1)
		QuickSort(arr, pivot+1, right)
	}
}

func getPartition(arr []int, left, right int) int {
	pivot := arr[left]

	l, r := left, right
	for l < r {
		for l < r && pivot <= arr[r] {
			r--
		}
		for l < r && pivot >= arr[l] {
			l++
		}
		if l < r { // 只有当索引没有交叉时才交换元素
			arr[l], arr[r] = arr[r], arr[l]
		}
	}

	arr[left], arr[l] = arr[l], arr[left] // 将枢轴正确交换到其最终位置

	return l
}
