// This is a Rust implementation of the Radix Sort algorithm.
pub fn radix_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }

    let mut exp = 1;
    while *arr.iter().max().unwrap_or(&0) / exp > 0 {
        counting_sort_by_digit(arr, exp);
        exp *= 10;
    }
}

fn counting_sort_by_digit(arr: &mut [i32], exp: i32) {
    let (mut count, mut output) = (vec![0; 10], vec![0; arr.len()]);

    for &v in arr.iter() {
        let digit = (v / exp) % 10;
        count[digit as usize] += 1;
    }

    for v in 1..10 {
        count[v] += count[v - 1];
    }

    for &v in arr.iter().rev() {
        let digit = (v / exp) % 10;
        output[count[digit as usize] - 1] = v;
        count[digit as usize] -= 1;
    }

    for (i, &v) in output.iter().enumerate() {
        arr[i] = v;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radix_sort() {
        let mut arr = [4, 2, 1, 3, 5];
        radix_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }
}
