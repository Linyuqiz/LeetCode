// This is an implementation of the counting sort algorithm in Rust.
pub fn counting_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }

    let max = *arr.iter().max().unwrap_or(&0);
    let mut count = vec![0; (max + 1) as usize];
    for &num in arr.iter() {
        count[num as usize] += 1;
    }

    let mut index = 0;
    for (i, &v) in count.iter().enumerate() {
        for _ in 0..v {
            arr[index] = i as i32;
            index += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counting_sort() {
        let mut arr = [4, 2, 1, 3, 5];
        counting_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }
}
