// This is a Rust implementation of the merge sort algorithm.
pub fn merge_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }

    let (left, right) = arr.split_at_mut(arr.len() / 2);
    merge_sort(left);
    merge_sort(right);

    merge(left, right);
}

fn merge(left: &mut [i32], right: &mut [i32]) {
    let mut merged = Vec::with_capacity(left.len() + right.len());

    let (mut i, mut j) = (0, 0);
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            merged.push(left[i]);
            i += 1;
        } else {
            merged.push(right[j]);
            j += 1;
        }
    }

    merged.extend_from_slice(&left[i..]);
    merged.extend_from_slice(&right[j..]);

    left.copy_from_slice(&merged[..left.len()]);
    right.copy_from_slice(&merged[left.len()..]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sort() {
        let mut arr = [4, 2, 1, 3, 5];
        merge_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }
}
