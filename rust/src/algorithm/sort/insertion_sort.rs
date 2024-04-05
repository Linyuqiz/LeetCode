// This is an implementation of the insertion sort algorithm in Rust.
pub fn insertion_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }

    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion_sort() {
        let mut arr = [4, 2, 1, 3, 5];
        insertion_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }
}
