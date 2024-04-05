// This is the implementation of the quick sort algorithm in Rust.
pub fn quick_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }

    let len = arr.len();
    let pivot_index = len / 2;
    let pivot = arr[pivot_index];

    let (mut left, mut right) = (0, len - 1);
    while left <= right {
        while arr[left] < pivot {
            left += 1;
        }
        while arr[right] > pivot {
            right -= 1;
        }

        if left <= right {
            arr.swap(left, right);
            left += 1;
            right -= 1;
        }
    }

    let (left_slice, right_slice) = arr.split_at_mut(left);
    quick_sort(left_slice);
    quick_sort(&mut right_slice[1..]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_sort() {
        let mut arr = [4, 2, 1, 3, 5];
        quick_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }
}
