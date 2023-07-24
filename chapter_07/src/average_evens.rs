pub fn find(data: &[i32]) -> Option<f32> {
    let mut sum = 0;
    let mut count = 0;

    for v in data {
        if v % 2 == 0 {
            sum += v;
            count += 1;
        }
    }
    match count {
        0 => None,
        _ => Some(sum as f32 / count as f32),
    }
}

//// Rust Extras
#[allow(dead_code)]
fn find_extra(data: &[i32]) -> Option<f32> {
    let evens: Vec<_> = data.iter().filter(|&x| x % 2 == 0).copied().collect();
    match evens.is_empty() {
        true => None,
        false => Some(evens.iter().sum::<i32>() as f32 / evens.len() as f32),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find() {
        assert_eq!(find(&[]), None);
        assert_eq!(find(&[4, 2, 7, 1, 3]), Some(3.0));
        assert_eq!(find(&[5, 4, 2, 9, 2, 2]), Some(2.5));
    }

    #[test]
    fn test_find_extra() {
        assert_eq!(find_extra(&[]), None);
        assert_eq!(find_extra(&[4, 2, 7, 1, 3]), Some(3.0));
        assert_eq!(find_extra(&[5, 4, 2, 9, 2, 2]), Some(2.5));
    }
}
