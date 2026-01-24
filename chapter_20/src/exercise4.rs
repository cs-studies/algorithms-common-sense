pub fn greatest_product(numbers: &[i32]) -> Option<i32> {
    if numbers.len() < 2 {
        return None;
    }

    let mut min1 = i32::MAX;
    let mut min2 = i32::MAX;
    let mut max1 = i32::MIN;
    let mut max2 = i32::MIN;

    for &n in numbers {
        if n < min1 {
            min2 = min1;
            min1 = n;
        } else if n < min2 {
            min2 = n;
        }

        if n > max1 {
            max2 = max1;
            max1 = n;
        } else if n > max2 {
            max2 = n;
        }
    }

    let p1 = min1.checked_mul(min2)?;
    let p2 = max1.checked_mul(max2)?;

    Some(p1.max(p2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greatest_product() {
        assert_eq!(greatest_product(&[]), None);
        assert_eq!(greatest_product(&[1]), None);
        assert_eq!(greatest_product(&[0, 0]), Some(0));
        assert_eq!(greatest_product(&[1, 0]), Some(0));
        assert_eq!(greatest_product(&[0, -1]), Some(0));
        assert_eq!(greatest_product(&[-1, -2]), Some(2));
        assert_eq!(greatest_product(&[-1, 2]), Some(-2));
        assert_eq!(greatest_product(&[3, 2, 5]), Some(15));
        assert_eq!(greatest_product(&[5, -10, 4]), Some(20));
        assert_eq!(greatest_product(&[-3, -2, -5]), Some(15));
        assert_eq!(greatest_product(&[3, 2, -5]), Some(6));
        assert_eq!(greatest_product(&[-3, 2, -5]), Some(15));
        assert_eq!(greatest_product(&[-3, 2, -5, 0]), Some(15));
        assert_eq!(greatest_product(&[-3, 2, -5, 10]), Some(20));
        assert_eq!(greatest_product(&[-3, 2, -5, 10, 0]), Some(20));
    }
}
