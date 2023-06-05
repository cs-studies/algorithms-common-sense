pub fn sample(data: &[i32]) -> Option<[i32; 3]> {
    let data_len = data.len();
    match data_len {
        0 => None,
        _ => Some([data[0], data[data_len / 2], data[data_len - 1]]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        assert_eq!(None, sample(&[]));
        assert_eq!(Some([2, 2, 2]), sample(&[2]));
        assert_eq!(Some([2, 4, 10]), sample(&[2, 4, 10]));
        assert_eq!(Some([2, 8, 10]), sample(&[2, 4, 8, 10]));
        assert_eq!(Some([2, 9, 11]), sample(&[2, 4, 8, 9, 10, 11]));
    }
}
