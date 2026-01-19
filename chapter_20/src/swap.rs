use std::collections::HashMap;

pub fn find_indices_1(data1: &[i32], data2: &[i32]) -> Option<(usize, usize)> {
    let sum1 = data1.iter().sum::<i32>();
    let sum2 = data2.iter().sum::<i32>();

    // We want: sum1 - v1 + v2 = sum2 - v2 + v1
    // Hence, v2 = v1 + (sum2 - sum1) / 2
    let diff = sum2 - sum1;
    if diff % 2 != 0 {
        return None;
    }

    let shift = diff / 2;

    for (i, &v1) in data1.iter().enumerate() {
        let counterpart = v1 + shift;
        for (j, &v2) in data2.iter().enumerate() {
            if v2 == counterpart {
                return Some((i, j));
            }
        }
    }
    None
}

pub fn find_indices_2(data1: &[i32], data2: &[i32]) -> Option<(usize, usize)> {
    let mut hm = HashMap::<i32, usize>::with_capacity(data1.len());
    let mut sum1 = 0;

    for (i, &v1) in data1.iter().enumerate() {
        sum1 += v1;
        hm.insert(v1, i);
    }

    let sum2 = data2.iter().sum::<i32>();

    // We want: sum1 - v1 + v2 = sum2 - v2 + v1
    // Hence, v1 = v2 + (sum1 - sum2) / 2
    let diff = sum1 - sum2;
    if diff % 2 != 0 {
        return None;
    }

    let shift = diff / 2;

    for (j, &v2) in data2.iter().enumerate() {
        if let Some(&i) = hm.get(&(v2 + shift)) {
            return Some((i, j));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_indices_1() {
        let a1 = [1, 2, 3];
        let a2 = [5, 4];
        assert_eq!(find_indices_1(&a1, &a2), None);

        let a1 = [1, 2, 3];
        let a2 = [6, 4];
        assert_eq!(find_indices_1(&a1, &a2).unwrap(), (1, 1));

        let a1 = [4, 6];
        let a2 = [1, 2, 3];
        assert_eq!(find_indices_1(&a1, &a2).unwrap(), (0, 1));
    }

    #[test]
    fn test_find_indices_2() {
        let a1 = [1, 2, 3];
        let a2 = [5, 4];
        assert_eq!(find_indices_2(&a1, &a2), None);

        let a1 = [1, 2, 3];
        let a2 = [6, 4];
        assert_eq!(find_indices_2(&a1, &a2).unwrap(), (1, 1));

        let a1 = [4, 6];
        let a2 = [1, 2, 3];
        assert_eq!(find_indices_2(&a1, &a2).unwrap(), (0, 1));
    }
}
