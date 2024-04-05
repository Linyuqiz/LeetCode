// This is an implementation of the bucket sort algorithm in Rust.
pub fn bucket_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }

    let max_val = *arr.iter().max().unwrap_or(&0);
    let min_val = *arr.iter().min().unwrap_or(&0);

    let bucket_size = ((max_val - min_val) / arr.len() as i32).max(1);
    let mut buckets = vec![Vec::new(); ((max_val - min_val) / bucket_size + 1) as usize];
    for &v in arr.iter() {
        buckets[((v - min_val) / bucket_size) as usize].push(v);
    }

    let mut index = 0;
    for mut bucket in buckets {
        bucket.sort();
        for num in bucket {
            arr[index] = num;
            index += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bucket_sort() {
        let mut arr = [4, 2, 1, 3, 5];
        bucket_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }
}
