// This is a Rust implementation of the bubble sort algorithm.
pub fn bubble_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }

    let n = arr.len();
    for i in 0..n {
        for j in i + 1..n {
            if arr[i] > arr[j] {
                arr.swap(i, j);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut arr = [4, 2, 1, 3, 5];
        bubble_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }
}
