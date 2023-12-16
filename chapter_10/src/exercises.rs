pub fn print_every_other(low: i32, high: i32) {
    if low > high {
        return;
    }
    println!("{low}");
    print_every_other(low + 2, high);
}

pub fn sum(low: i32, high: i32) -> i32 {
    if low > high {
        return 0;
    }
    high + sum(low, high - 1)
}

#[derive(Debug)]
pub enum Recur {
    Leaf(i32),
    Node(Vec<Recur>),
}

pub fn print_all_numbers(data: Vec<Recur>) {
    for element in data {
        match element {
            Recur::Leaf(leaf) => println!("leaf: {:?}", leaf),
            Recur::Node(node) => print_all_numbers(node),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        assert_eq!(sum(1, 10), 55);
        assert_eq!(sum(2, 1), 0);
    }
}
