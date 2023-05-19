pub fn greatest_product(data: &[i32]) -> Option<i32> {
    if data.len() < 2 {
        return None;
    }
    let mut result = data[0] * data[1];
    for (i, m) in data.iter().enumerate() {
        for (j, n) in data.iter().enumerate() {
            if i != j && m * n > result {
                result = m * n;
            }
        }
    }
    Some(result)
}

#[allow(dead_code)]
pub fn greatest_product_sorted(data: &[i32]) -> Option<i32> {
    if data.len() < 2 {
        return None;
    }
    let mut sorted = data.to_owned();
    sorted.sort();
    let i = sorted.len() - 1;
    Some(sorted[i] * sorted[i - 1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greatest_product() {
        assert_eq!(None, greatest_product(&[]));
        assert_eq!(Some(24), greatest_product(&[4, 1, 6, 2]));
    }

    #[test]
    fn test_greatest_product_sorted() {
        assert_eq!(None, greatest_product_sorted(&[]));
        assert_eq!(Some(24), greatest_product_sorted(&[4, 1, 6, 2]));
    }

}
