pub fn greatest_product(data: &mut [u32]) -> Option<u32> {
    if data.len() < 3 {
        return None;
    }
    data.sort_unstable_by(|a, b| b.cmp(a));
    data[0]
        .checked_mul(data[1])
        .and_then(|p| p.checked_mul(data[2]))
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
