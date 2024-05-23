pub fn greatest_product(data: &mut [u32]) -> Option<u32> {
    if data.len() < 3 {
        return None;
    }
    data.sort_unstable_by(|a, b| b.cmp(a));
    data[0]
        .checked_mul(data[1])
        .and_then(|p| p.checked_mul(data[2]))
}

pub fn find_missing_number(data: &mut [u32]) -> Option<u32> {
    data.sort_unstable();
    for (i, n) in data.iter().enumerate() {
        if i as u32 != *n {
            return Some(i as u32);
        }
    }
    None
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

    #[test]
    fn test_find_missing_number() {
        assert_eq!(find_missing_number(&mut []), None);
        assert_eq!(find_missing_number(&mut [0]), None);
        assert_eq!(find_missing_number(&mut [1]), Some(0));
        assert_eq!(find_missing_number(&mut [1, 0]), None);
        assert_eq!(find_missing_number(&mut [0, 2]), Some(1));
        assert_eq!(find_missing_number(&mut [0, 2, 1]), None);
        assert_eq!(find_missing_number(&mut [3, 0, 1]), Some(2));
    }
}
