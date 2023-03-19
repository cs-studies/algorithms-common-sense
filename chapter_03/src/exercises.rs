#![allow(dead_code)]

fn is_leap_year(year: u16) -> bool {
    if year % 100 == 0 {
        year % 400 == 0
    } else {
        year % 4 == 0
    }
}

fn array_sum(arr: &[i32]) -> i32 {
    let mut sum: i32 = 0;
    for i in arr {
        sum = sum.checked_add(*i).expect("We've got an overflow!");
    }
    sum
}

fn chessboard_space(grains: u32) -> u32 {
    let mut squares: u32 = 1;
    let mut placed_grains = 1;
    while placed_grains < grains {
        placed_grains = placed_grains.checked_mul(2).expect("Grains overflow!");
        squares = squares.checked_add(1).expect("Gimmi a bigger board!");
    }
    squares
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_leap_year() {
        assert!(!is_leap_year(1900));
        assert!(!is_leap_year(2300));
        assert!(is_leap_year(2000));
        assert!(is_leap_year(2400));
        assert!(!is_leap_year(2023));
        assert!(is_leap_year(2024));
    }

    #[test]
    fn test_array_sum() {
        assert_eq!(0, array_sum(&[]));
        assert_eq!(7, array_sum(&[4, 1, 2]));
        assert_eq!(118, array_sum(&[100, 11, 2, 5]));
        assert_eq!(5, array_sum(&[2, 8, -15, 10]));
        assert_eq!(i32::MAX, array_sum(&[(i32::MAX - 1), 1]));
    }

    #[test]
    #[should_panic]
    fn test_array_sum_panics() {
        array_sum(&[i32::MAX, 1]);
    }

    //// Rust Extras

    #[test]
    fn test_array_sum_idiomatic() {
        assert_eq!(0, [].iter().sum());
        assert_eq!(7, [4, 1, 2].iter().sum());
        assert_eq!(118, [100, 11, 2, 5].iter().sum());
        assert_eq!(5, [2, 8, -15, 10].iter().sum());
        assert_eq!(i32::MAX, [(i32::MAX - 1), 1].iter().sum());
    }

    #[test]
    #[should_panic]
    fn test_array_sum_idiomatic_panics() {
        [i32::MAX, 1].iter().sum::<i32>();
    }
    //// End of Rust Extras

    #[test]
    fn test_chessboard_space() {
        assert_eq!(1, chessboard_space(1));
        assert_eq!(2, chessboard_space(2));
        assert_eq!(3, chessboard_space(4));
        assert_eq!(4, chessboard_space(8));
        assert_eq!(5, chessboard_space(16));
    }

    #[test]
    fn test_chessboard_space_panics() {
        chessboard_space(3);
    }
}
