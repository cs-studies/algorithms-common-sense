pub fn is_leap_year(year: u16) -> bool {
    if year % 100 == 0 {
        year % 400 == 0
    } else {
        year % 4 == 0
    }
}

pub fn array_sum(arr: &[i32]) -> i32 {
    let mut sum: i32 = 0;
    for i in arr {
        sum = sum
            .checked_add(*i)
            .expect("simple examples should not overflow memory");
    }
    sum
}

pub fn chessboard_space(grains: u32) -> u8 {
    println!("Given {grains} grains");

    let mut square: u8 = 1;
    let mut placed_grains = 1;

    while placed_grains < grains {
        println!("while: {placed_grains} < {grains}");

        print!("placed_grains: {placed_grains} * 2 = ");
        placed_grains = placed_grains
            .checked_mul(2)
            .expect("simple examples should not overflow memory");
        println!("{placed_grains}");

        print!("square: {square} + 1 = ");
        // Even with u32::MAX grains, we'd use only 33 squares!
        square = square
            .checked_add(1)
            .expect("simple examples should not overflow memory");
        println!("{square}");
    }
    square
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
        assert_eq!(array_sum(&[]), 0);
        assert_eq!(array_sum(&[4, 1, 2]), 7);
        assert_eq!(array_sum(&[100, 11, 2, 5]), 118);
        assert_eq!(array_sum(&[2, 8, -15, 10]), 5);
        assert_eq!(array_sum(&[(i32::MAX - 1), 1]), i32::MAX);
    }

    #[test]
    #[should_panic]
    fn test_array_sum_panics() {
        array_sum(&[i32::MAX, 1]);
    }

    //// Rust Extras

    #[test]
    fn test_array_sum_idiomatic() {
        assert_eq!([].iter().sum::<i32>(), 0);
        assert_eq!([4, 1, 2].iter().sum::<i32>(), 7);
        assert_eq!([100, 11, 2, 5].iter().sum::<i32>(), 118);
        assert_eq!([2, 8, -15, 10].iter().sum::<i32>(), 5);
        assert_eq!([(i32::MAX - 1), 1].iter().sum::<i32>(), i32::MAX);
    }

    #[test]
    #[should_panic]
    fn test_array_sum_idiomatic_panics() {
        [i32::MAX, 1].iter().sum::<i32>();
    }
    //// End of Rust Extras

    #[test]
    fn test_chessboard_space() {
        assert_eq!(chessboard_space(1), 1);
        assert_eq!(chessboard_space(2), 2);
        assert_eq!(chessboard_space(3), 3);
        assert_eq!(chessboard_space(4), 3);
        assert_eq!(chessboard_space(5), 4);
        assert_eq!(chessboard_space(8), 4);
        assert_eq!(chessboard_space(9), 5);
        assert_eq!(chessboard_space(16), 5);
        assert_eq!(chessboard_space(17), 6);
        assert_eq!(chessboard_space((u32::MAX / 2) + 1), 32);
    }

    #[test]
    #[should_panic]
    fn test_chessboard_space_panics() {
        chessboard_space(u32::MAX);
    }

    #[test]
    #[should_panic]
    fn test_chessboard_space_panics_2() {
        chessboard_space((u32::MAX / 2) + 2);
    }
}
