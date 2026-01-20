pub fn max_segment_sum(data: &[i32]) -> Result<i32, &'static str> {
    let (&first, rest) = data
        .split_first()
        .ok_or("data must contain at least one element")?;

    let mut current = first;
    let mut greatest = first;

    for &v in rest {
        current = (current + v).max(v);
        greatest = greatest.max(current);
    }

    Ok(greatest)
}

pub fn upward_trend(data: &[i32]) -> Result<bool, &'static str> {
    let (mut first, rest) = match data.split_first() {
        Some((first, rest)) => (*first, rest),
        None => return Ok(false),
    };

    let mut second = i32::MAX;

    for &v in rest {
        if v <= first {
            first = v;
        } else if v <= second {
            second = v;
        } else {
            return Ok(true);
        }
    }

    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_segment_sum() {
        assert_eq!(
            max_segment_sum(&[]).unwrap_err(),
            "data must contain at least one element"
        );

        assert_eq!(max_segment_sum(&[1]), Ok(1));
        assert_eq!(max_segment_sum(&[1, 2]), Ok(3));
        assert_eq!(max_segment_sum(&[1, 2, 3]), Ok(6));
        assert_eq!(max_segment_sum(&[1, 2, 3, 4]), Ok(10));
        assert_eq!(max_segment_sum(&[3, -4, 4, -3, 5, -9]), Ok(6));
        assert_eq!(max_segment_sum(&[1, 1, 0, -3, 5]), Ok(5));
        assert_eq!(max_segment_sum(&[5, -2, 3, -8, 4]), Ok(6));
        assert_eq!(max_segment_sum(&[2, -3, 1, 2, -1]), Ok(3));
        assert_eq!(max_segment_sum(&[5, -8, 2, 1, 0]), Ok(5));
        assert_eq!(max_segment_sum(&[-2, -1]), Ok(-1));
    }

    #[test]
    fn test_has_upward_trend() {
        assert_eq!(upward_trend(&[]), Ok(false));
        assert_eq!(upward_trend(&[1]), Ok(false));
        assert_eq!(upward_trend(&[2, 1]), Ok(false));
        assert_eq!(upward_trend(&[1, 2]), Ok(false));
        assert_eq!(upward_trend(&[8, 9, 7, 10]), Ok(true));
        assert_eq!(upward_trend(&[5, 2, 8, 4, 3, 7]), Ok(true));
    }
}
