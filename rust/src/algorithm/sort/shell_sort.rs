// This is a Rust implementation of the Shell Sort algorithm.
pub fn shell_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }

    let n = arr.len();
    let mut gap = n / 2;
    while gap > 0 {
        for i in gap..n {
            let mut j = i;
            while j >= gap && arr[j - gap] > arr[j] {
                arr.swap(j - gap, j);
                j -= gap;
            }
        }
        gap /= 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_sort() {
        let mut arr = [4, 2, 5, 3, 1];
        shell_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }
}
