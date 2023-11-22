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
    let (mut a_idx, mut b_idx) = (0, 0);

    while a_idx < a.len() && b_idx < b.len() {
        if a[a_idx] < b[b_idx] {
            result.push(a[a_idx]);
            a_idx += 1;
        } else {
            result.push(b[b_idx]);
            b_idx += 1;
        }
    }

    result.extend_from_slice(&a[a_idx..]);
    result.extend_from_slice(&b[b_idx..]);

    result
}

// haystack.contains(needle)
pub fn find_needle(needle: &str, haystack: &str) -> bool {
    if needle.is_empty() {
        return true;
    }

    let needle_len = needle.chars().count();
    let mut count = 0;

    for c in haystack.chars() {
        if c == needle.chars().nth(count).unwrap() {
            count += 1;
            if count == needle_len {
                return true;
            }
        } else {
            count = 0;
        }
    }
    false
}

pub fn largest_product(data: &[i32]) -> Option<i32> {
    let data_len = data.len();
    if data_len < 3 {
        return None;
    }
    let mut result = i32::MIN;

    for i in 0..data_len {
        for j in (i + 1)..data_len {
            for k in (j + 1)..data_len {
                let product = data[i]
                    .checked_mul(data[j])
                    .and_then(|p| p.checked_mul(data[k]));
                match product {
                    Some(p) => result = result.max(p),
                    None => return None
                }
            }
        }
    }
    Some(result)
}

pub fn pick_resume(resumes: &[i32]) -> Option<i32> {
    if resumes.is_empty() {
        return None;
    }
    let mut remove_top = true;
    let mut result = resumes;

    while result.len() > 1 {
        let midpoint = result.len() / 2;
        result = if remove_top {
            &result[0..midpoint]
        } else {
            &result[midpoint..]
        };
        remove_top = !remove_top;
    }
    Some(result[0])
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

    #[test]
    fn test_find_needle() {
        assert!(find_needle("", ""));
        assert!(find_needle("", "project"));
        assert!(!find_needle("pro", ""));
        assert!(find_needle("pro", "project"));
        assert!(find_needle("om", "home"));
        assert!(!find_needle("om", "project"));
    }

    #[test]
    fn test_largest_product() {
        assert_eq!(largest_product(&[]), None);
        assert_eq!(largest_product(&[1]), None);
        assert_eq!(largest_product(&[1, 2]), None);
        assert_eq!(largest_product(&[1, 2, i32::MAX]), None);
        assert_eq!(largest_product(&[1, 2, 3]), Some(6));
        assert_eq!(largest_product(&[1, 2, 2, -3]), Some(4));
        assert_eq!(largest_product(&[5, 2, 3, 4]), Some(60));
        assert_eq!(largest_product(&[-1, -2, -3]), Some(-6));
    }

    #[test]
    fn test_pick_resume() {
        assert_eq!(pick_resume(&[]), None);
        assert_eq!(pick_resume(&[1]), Some(1));
        assert_eq!(pick_resume(&[2, 3]), Some(2));
        assert_eq!(pick_resume(&[4, 5, 6]), Some(4));
        assert_eq!(pick_resume(&[7, 8, 9, 10]), Some(8));
    }
}
