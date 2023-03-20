mod exercises;

fn main() {
    println!("\n*** Chapter 03 ***\n");

    print_things(&["apples", "baboons", "cribs", "dulcimbers"]);
    print_things(&["a", "b", "c", "d", "e", "f", "g"]);

    println!("149 is a prime number: {}", is_prime(149));

    //// Exercises
    println!("\n*** Exercises ***\n");

    println!("2025 is a leap year: {}\n", exercises::is_leap_year(2025));

    let arr = [2, 5, 3];
    println!("array_sum: {:?} = {}\n", &arr, exercises::array_sum(&arr));

    exercises::chessboard_space(3);
    // For those interested to see the efficiency of doubling.
    // exercises::chessboard_space(u32::MAX / 2);
}

fn print_things(things: &[&str]) {
    for thing in things {
        println!("Here's a thing: {thing}");
    }
    println!("The number of steps is {}\n", things.len());
}

fn is_prime(number: i32) -> bool {
    for i in 2..number {
        if number % i == 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::is_prime;

    #[test]
    fn test_is_prime() {
        assert!(is_prime(13));
        assert!(!is_prime(14));
        assert!(is_prime(1009));
    }
}
