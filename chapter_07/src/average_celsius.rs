pub fn find(data_fahrenheit: &[f32]) -> Option<f32> {
    if data_fahrenheit.is_empty() {
        return None;
    }
    let mut data_celsius = Vec::new();
    for f in data_fahrenheit {
        let converted = (f - 32.0) / 1.8;
        data_celsius.push(converted);
    }
    let mut sum = 0.0;
    for c in data_celsius.iter() {
        sum += c;
    }
    Some(sum / data_celsius.len() as f32)
}

//// Rust Extras
#[allow(dead_code)]
fn find_extra(data_fahrenheit: &[f32]) -> Option<f32> {
    let data_len = data_fahrenheit.len();
    match data_len {
        0 => None,
        _ => {
            let sum: f32 = data_fahrenheit
                .iter()
                .copied()
                .map(|f| (f - 32.0) / 1.8)
                .sum();
            Some(sum / data_len as f32)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find() {
        assert_eq!(None, find(&[]));
        assert_eq!(Some(0.0), find(&[32.0]));
        assert_eq!(Some(30.0), find(&[68.0, 104.0]));
    }

    #[test]
    fn test_find_extra() {
        assert_eq!(None, find_extra(&[]));
        assert_eq!(Some(0.0), find_extra(&[32.0]));
        assert_eq!(Some(30.0), find_extra(&[68.0, 104.0]));
    }
}
