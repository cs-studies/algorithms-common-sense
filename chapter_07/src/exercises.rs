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
}