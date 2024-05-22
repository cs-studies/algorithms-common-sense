pub fn greatest_product(data: &mut [u32]) -> Option<u32> {
    let data_len = data.len();
    if data_len < 3 {
        return None;
    }
    data.sort();

    data[data_len - 1]
        .checked_mul(data[data_len - 2])
        .and_then(|p| p.checked_mul(data[data_len - 3]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greatest_product() {
        assert_eq!(greatest_product(&mut []), None);
        assert_eq!(greatest_product(&mut [4]), None);
        assert_eq!(greatest_product(&mut [4, 1]), None);
        assert_eq!(greatest_product(&mut [4, 1, 6]), Some(24));
        assert_eq!(greatest_product(&mut [4, 1, 6, 2]), Some(48));
        assert_eq!(greatest_product(&mut [1, 2, u32::MAX]), None);
    }
}
