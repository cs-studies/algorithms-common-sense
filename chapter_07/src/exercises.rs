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

//// Rust Extras
#[allow(dead_code)]
fn is_one_hundred_sum_extra(data: &[i32]) -> bool {
    if data.len() < 2 {
        return false;
    }
    data.iter()
        .zip(data.iter().rev())
        .any(|(&a, &b)| a + b == 100)
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
    }

    #[test]
    fn test_is_one_hundred_sum_extra() {
        assert_eq!(is_one_hundred_sum_extra(&[]), false);
        assert_eq!(is_one_hundred_sum_extra(&[100]), false);
        assert_eq!(is_one_hundred_sum_extra(&[1, 2]), false);
        assert_eq!(is_one_hundred_sum_extra(&[1, 99]), true);
        assert_eq!(is_one_hundred_sum_extra(&[1, 100, 2]), false);
        assert_eq!(is_one_hundred_sum_extra(&[1, 97, 3, 5]), true);
    }
}
