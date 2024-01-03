pub fn count_chars(data: &[&str]) -> usize {
    if data.is_empty() {
        return 0;
    }
    data[0].len() + count_chars(&data[1..])
}

pub fn find_evens(data: &[i32]) -> Vec<i32> {
    if data.is_empty() {
        return Vec::<i32>::new();
    }
    let mut evens = find_evens(&data[1..]);
    if data[0] % 2 == 0 {
        evens.insert(0, data[0]);
    }
    evens
}

pub fn triangular_number(at: u16) -> u16 {
    if at == 0 {
        return 0;
    }
    at.checked_add(triangular_number(at - 1))
        .expect("simple examples should not overflow memory")
}

pub fn index_x(s: &str) -> usize {
    if s.is_empty() {
        panic!("The string must contain 'x'");
    }
    let (first, rest) = s.split_at(1);
    if first == "x" {
        return 0;
    }
    1 + index_x(rest)
}

pub fn unique_paths(rows: u8, columns: u8) -> u16 {
    if rows < 1 || columns < 1 {
        panic!("rows and columns must be positive numbers");
    }
    if rows == 1 || columns == 1 {
        return 1;
    }
    unique_paths(rows - 1, columns)
        .checked_add(unique_paths(rows, columns - 1))
        .expect("simple examples should not overflow memory")
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

    #[test]
    fn test_triangular_number() {
        assert_eq!(triangular_number(0), 0);
        assert_eq!(triangular_number(1), 1);
        assert_eq!(triangular_number(2), 3);
        assert_eq!(triangular_number(3), 6);
        assert_eq!(triangular_number(4), 10);
        assert_eq!(triangular_number(5), 15);
        assert_eq!(triangular_number(6), 21);
    }

    #[test]
    #[should_panic]
    fn test_triangular_number_panics() {
        triangular_number(u16::MAX / 2);
    }

    #[test]
    fn test_index_x() {
        assert_eq!(index_x("x"), 0);
        assert_eq!(index_x("abx"), 2);
    }

    #[test]
    #[should_panic]
    fn test_index_x_panics_empty_string() {
        index_x("");
    }

    #[test]
    #[should_panic]
    fn test_index_x_panics_no_x() {
        index_x("abc");
    }

    #[test]
    fn test_unique_paths() {
        assert_eq!(unique_paths(1, 1), 1);
        assert_eq!(unique_paths(2, 1), 1);
        assert_eq!(unique_paths(1, 2), 1);
        assert_eq!(unique_paths(2, 2), 2);
        assert_eq!(unique_paths(3, 1), 1);
        assert_eq!(unique_paths(1, 3), 1);
        assert_eq!(unique_paths(3, 2), 3);
        assert_eq!(unique_paths(2, 3), 3);
    }

    #[test]
    #[should_panic]
    fn test_unique_paths_00() {
        unique_paths(0, 0);
    }

    #[test]
    #[should_panic]
    fn test_unique_paths_01() {
        unique_paths(0, 1);
    }

    #[test]
    #[should_panic]
    fn test_unique_paths_10() {
        unique_paths(1, 0);
    }
}
