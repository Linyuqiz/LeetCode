// This is a Rust implementation of the heap sort algorithm.
pub fn heap_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }

    let n = arr.len();
    for i in (0..n / 2).rev() {
        heapify(arr, n, i);
    }

    for i in (1..n).rev() {
        arr.swap(0, i);
        heapify(&mut arr[..i], i, 0);
    }
}

fn heapify(arr: &mut [i32], n: usize, mut i: usize) {
    let mut largest = i;
    loop {
        let (l, r) = (2 * i + 1, 2 * i + 2);

        if l < n && arr[l] > arr[largest] {
            largest = l;
        }

        if r < n && arr[r] > arr[largest] {
            largest = r;
        }

        if largest == i {
            break;
        }

        arr.swap(i, largest);
        i = largest;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heap_sort() {
        let mut arr = [4, 1, 3, 2, 5];
        heap_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }
}
