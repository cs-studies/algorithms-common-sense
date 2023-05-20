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
fn greatest_product_sorted(data: &[i32]) -> Option<i32> {
    if data.len() < 2 {
        return None;
    }
    let mut sorted = data.to_owned();
    sorted.sort();
    let i = sorted.len() - 1;
    Some(sorted[i] * sorted[i - 1])
}

pub fn greatest_number(data: &[i32]) -> Option<i32> {
    if data.is_empty() {
        return None;
    }
    let mut greatest = data[0];
    for v in data {
        if v > &greatest {
            greatest = *v;
        }
    }
    Some(greatest)
}

//// Rust Extras
#[allow(dead_code)]
fn greatest_number_extra(data: &[i32]) -> Option<&i32> {
    data.iter().max()
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

    #[test]
    fn test_greatest_number() {
        assert_eq!(None, greatest_number(&[]));
        assert_eq!(Some(6), greatest_number(&[4, 1, 6, 2]));
    }

    #[test]
    fn test_greatest_number_extra() {
        assert_eq!(None, greatest_number_extra(&[]));
        assert_eq!(Some(&6), greatest_number_extra(&[4, 1, 6, 2]));
    }
}
