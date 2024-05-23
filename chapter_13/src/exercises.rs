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

pub fn max_n2(data: &[i32]) -> Option<i32> {
    let mut max = None;
    for i in 0..data.len() {
        let mut i_is_max = true;
        for j in 0..data.len() {
            if data[j] > data[i] {
                i_is_max = false;
            }
        }
        if i_is_max {
            max = Some(data[i]);
        }
    }
    max
}

pub fn max_nlogn(data: &mut [i32]) -> Option<i32> {
    data.sort_unstable_by(|a, b| b.cmp(a));
    data.first().copied()
}

pub fn max_n(data: &[i32]) -> Option<i32> {
    let mut max = data.first();
    for n in data.iter().skip(1) {
        if n > max.unwrap() {
            max = Some(n);
        }
    }
    max.copied()
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

    #[test]
    fn test_max_n2() {
        assert_eq!(max_n2(&[]), None);
        assert_eq!(max_n2(&[-1]), Some(-1));
        assert_eq!(max_n2(&[3, 5, -10, 90, 20]), Some(90));
    }

    #[test]
    fn test_max_nlogn() {
        assert_eq!(max_nlogn(&mut []), None);
        assert_eq!(max_nlogn(&mut [-1]), Some(-1));
        assert_eq!(max_nlogn(&mut [3, 5, -10, 90, 20]), Some(90));
    }

    #[test]
    fn test_max_n() {
        assert_eq!(max_n(&[]), None);
        assert_eq!(max_n(&[-1]), Some(-1));
        assert_eq!(max_n(&[3, 5, -10, 90, 20]), Some(90));
    }
}
