package main

func MergeSort(arr []int) []int {
	if len(arr) <= 1 {
		return arr
	}

	return merge(MergeSort(arr[:len(arr)/2]), MergeSort(arr[len(arr)/2:]))
}

func merge(left, right []int) []int {
	mergeList := make([]int, len(left)+len(right))

	i, j, k := 0, 0, 0
	for i < len(left) && j < len(right) {
		if left[i] < right[j] {
			mergeList[k] = left[i]
			i++
		} else {
			mergeList[k] = right[j]
			j++
		}
		k++
	}

	for i < len(left) {
		mergeList[k] = left[i]
		i++
		k++
	}
	for j < len(right) {
		mergeList[k] = right[j]
		j++
		k++
	}

	return mergeList
}
