// This is a Rust implementation of the selection sort algorithm.
pub fn selection_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }

    let n = arr.len();
    for i in 0..n {
        let mut min_idx = i;
        for j in i + 1..n {
            if arr[j] < arr[min_idx] {
                min_idx = j;
            }
        }
        arr.swap(i, min_idx);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_sort() {
        let mut arr = [4, 2, 1, 3, 5];
        selection_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }
}
