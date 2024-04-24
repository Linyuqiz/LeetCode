package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestHeapSort(t *testing.T) {
	arr := []int{5, 2, 8, 3, 9, 2, 5, 3, 4, 1, 7, 4, 6}
	sortedArr := []int{1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 7, 8, 9}

	HeapSort(arr)
	if !assert.EqualValues(t, sortedArr, arr) {
		t.Errorf("Expected %v, got %v", sortedArr, arr)
	}
}
