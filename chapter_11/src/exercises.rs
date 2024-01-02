pub fn count_chars(data: &[&str]) -> usize {
    if data.is_empty() {
        return 0
    }
    data[0].len() + count_chars(&data[1..])
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
}
