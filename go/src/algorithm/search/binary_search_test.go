package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestBinarySearch(t *testing.T) {
	arr := []int{1, 3, 5, 7, 9, 11, 13}

	getIndex := BinarySearch(arr, 5)
	if !assert.Equal(t, 2, getIndex) {
		t.Errorf("BinarySearch failed: expected 3, got [%d]", getIndex)
	}

}
