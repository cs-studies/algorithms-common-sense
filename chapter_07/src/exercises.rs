pub fn is_one_hundred_sum(data: &[i32]) -> bool {
    let data_len = data.len();
    if data_len < 2 {
        return false;
    }
    for i in 0..(data_len / 2) {
        let sum = data[i].checked_add(data[data_len - 1 - i]);
        if let Some(100) = sum {
            return true;
        }
    }
    false
}

pub fn merge_sorted(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut result = Vec::<i32>::new();
    let a_len = a.len();
    let b_len = b.len();
    let (mut a_idx, mut b_idx) = (0, 0);

    while a_idx < a_len || b_idx < b_len {
        if a_idx == a_len {
            result.push(b[b_idx]);
            b_idx += 1;
        } else if b_idx == b_len {
            result.push(a[a_idx]);
            a_idx += 1;
        } else if a[a_idx] < b[b_idx] {
            result.push(a[a_idx]);
            a_idx += 1;
        } else {
            result.push(b[b_idx]);
            b_idx += 1;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_one_hundred_sum() {
        assert_eq!(is_one_hundred_sum(&[]), false);
        assert_eq!(is_one_hundred_sum(&[100]), false);
        assert_eq!(is_one_hundred_sum(&[1, 2]), false);
        assert_eq!(is_one_hundred_sum(&[1, 99]), true);
        assert_eq!(is_one_hundred_sum(&[1, 100, 2]), false);
        assert_eq!(is_one_hundred_sum(&[1, 97, 3, 5]), true);
        assert_eq!(is_one_hundred_sum(&[1, 103, -3, 5]), true);
    }

    #[test]
    fn test_merge_sorted() {
        assert_eq!(merge_sorted(&[], &[]), vec![]);
        assert_eq!(merge_sorted(&[1], &[]), vec![1]);
        assert_eq!(merge_sorted(&[], &[2]), vec![2]);
        assert_eq!(merge_sorted(&[1], &[2]), vec![1, 2]);
        assert_eq!(merge_sorted(&[2], &[1]), vec![1, 2]);
        assert_eq!(merge_sorted(&[2], &[-1]), vec![-1, 2]);
        assert_eq!(
            merge_sorted(&[11, 20, 30], &[-10, 15]),
            vec![-10, 11, 15, 20, 30]
        );
    }
}
