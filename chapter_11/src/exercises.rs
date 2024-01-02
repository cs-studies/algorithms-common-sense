pub fn count_chars(data: &[&str]) -> usize {
    if data.is_empty() {
        return 0
    }
    data[0].len() + count_chars(&data[1..])
}

pub fn find_evens(data: &[i32]) -> Vec<i32> {
    if data.is_empty() {
        return Vec::<i32>::new();
    }
    let mut evens = find_evens(&data[1..]);
    if data[0] % 2 == 0  {
        evens.insert(0, data[0]);
    }
    evens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_chars() {
        assert_eq!(count_chars(&[""]), 0);
        assert_eq!(count_chars(&["a"]), 1);
        assert_eq!(count_chars(&["a", "b"]), 2);
        assert_eq!(count_chars(&["ab", "c", "de"]), 5);
    }

    #[test]
    fn test_find_evens() {
        let empty_vec = Vec::<i32>::new();
        assert_eq!(find_evens(&empty_vec), empty_vec);
        assert_eq!(find_evens(&[1]), empty_vec);
        assert_eq!(find_evens(&[2]), vec![2]);
        assert_eq!(find_evens(&[1, 2]), vec![2]);
        assert_eq!(find_evens(&[10, 17, 4]), vec![10, 4]);
    }
}
