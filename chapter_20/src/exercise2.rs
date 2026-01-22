use std::collections::HashSet;

const ERR_NO_MISSING_NUMBER: &str = "no missing number found";
const ERR_UNEXPECTED_DATA: &str = "unexpected data set";

pub fn find_missing_number_1(data: &[u32]) -> Result<u32, &'static str> {
    let hs: HashSet<u32> = data.iter().copied().collect();
    for i in 0..data.len() as u32 {
        if !hs.contains(&i) {
            return Ok(i);
        }
    }
    Err(ERR_NO_MISSING_NUMBER)
}

pub fn find_missing_number_2(data: &[u32]) -> Result<u32, &'static str> {
    let n = data.len() as u32;
    let actual: u32 = data.iter().sum();
    let ideal: u32 = (1..=n).sum();

    if actual > ideal {
        return Err(ERR_UNEXPECTED_DATA);
    }
    let missing = ideal - actual;
    if missing < n {
        Ok(missing)
    } else {
        Err(ERR_NO_MISSING_NUMBER)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_missing_number_1() {
        assert_eq!(
            find_missing_number_1(&[]).unwrap_err(),
            ERR_NO_MISSING_NUMBER
        );
        assert_eq!(
            find_missing_number_1(&[0]).unwrap_err(),
            ERR_NO_MISSING_NUMBER
        );
        assert_eq!(
            find_missing_number_1(&[0, 1]).unwrap_err(),
            ERR_NO_MISSING_NUMBER
        );

        assert_eq!(find_missing_number_1(&[0, 2]), Ok(1));
        assert_eq!(find_missing_number_1(&[2, 1]), Ok(0));
        assert_eq!(find_missing_number_1(&[1, 3, 0, 4]), Ok(2));
        assert_eq!(find_missing_number_1(&[4, 5, 0, 2, 1]), Ok(3));

        assert_eq!(find_missing_number_1(&[3, 2]), Ok(0));
    }

    #[test]
    fn test_find_missing_number_2() {
        assert_eq!(
            find_missing_number_2(&[]).unwrap_err(),
            ERR_NO_MISSING_NUMBER
        );
        assert_eq!(
            find_missing_number_2(&[0]).unwrap_err(),
            ERR_NO_MISSING_NUMBER
        );
        assert_eq!(
            find_missing_number_2(&[0, 1]).unwrap_err(),
            ERR_NO_MISSING_NUMBER
        );

        assert_eq!(find_missing_number_2(&[0, 2]), Ok(1));
        assert_eq!(find_missing_number_2(&[2, 1]), Ok(0));
        assert_eq!(find_missing_number_2(&[1, 3, 0, 4]), Ok(2));
        assert_eq!(find_missing_number_2(&[4, 5, 0, 2, 1]), Ok(3));

        assert_eq!(
            find_missing_number_2(&[3, 2]).unwrap_err(),
            ERR_UNEXPECTED_DATA
        );
    }
}
