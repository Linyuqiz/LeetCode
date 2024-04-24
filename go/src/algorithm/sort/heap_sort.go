package main

func HeapSort(arr []int) {
	length := len(arr)
	for i := length/2 - 1; i >= 0; i-- {
		heapify(arr, length, i)
	}
	for i := length - 1; i > 0; i-- {
		arr[0], arr[i] = arr[i], arr[0]
		heapify(arr, i, 0)
	}
}

func heapify(arr []int, n int, i int) {
	largest := i

	l, r := i*2+1, i*2+2
	if l < n && arr[l] > arr[largest] {
		largest = l
	}
	if r < n && arr[r] > arr[largest] {
		largest = r
	}

	if largest != i {
		arr[i], arr[largest] = arr[largest], arr[i]
		heapify(arr, n, largest)
	}
}
