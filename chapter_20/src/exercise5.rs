use std::collections::HashMap;

pub fn sort_97_to_99(readings: &[f32]) -> Vec<f32> {
    let min = 970;
    let max = 990;

    let mut hm = HashMap::<i32, i32>::new();
    for &t in readings {
        let key = (t * 10.0).round() as i32;
        if key >= min && key <= max {
            hm.entry(key).and_modify(|c| *c += 1).or_insert(1);
        }
    }

    let mut result = Vec::with_capacity(readings.len());
    for i in min..=max {
        if let Some(&count) = hm.get(&i) {
            for _ in 0..count {
                result.push(i as f32 / 10.0)
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_temperatures() {
        assert_eq!(sort_97_to_99(&[]), vec![]);
        assert_eq!(sort_97_to_99(&[96.9]), vec![]);
        assert_eq!(sort_97_to_99(&[96.9, 99.1]), vec![]);
        assert_eq!(sort_97_to_99(&[98.5]), vec![98.5]);
        assert_eq!(sort_97_to_99(&[98.5, 98.5]), vec![98.5, 98.5]);
        assert_eq!(sort_97_to_99(&[98.5, 97.2]), vec![97.2, 98.5]);
        assert_eq!(sort_97_to_99(&[98.5, 97.2, 100.1]), vec![97.2, 98.5]);
        assert_eq!(
            sort_97_to_99(&[98.5, 99.0, 97.0, 99.0, 97.4]),
            vec![97.0, 97.4, 98.5, 99.0, 99.0]
        );
    }
}
