fn main() {
    println!("\n*** Chapter 03 ***\n");

    print_things(&["apples", "baboons", "cribs", "dulcimbers"]);
    print_things(&["a", "b", "c", "d", "e", "f", "g"]);

    println!("149 is a prime number: {}", is_prime(149));
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
    use super::*;

    #[test]
    fn test_is_prime() {
        assert!(is_prime(13));
        assert!(!is_prime(14));
        assert!(is_prime(1009));
    }
}
