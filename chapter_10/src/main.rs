use exercises::Recur::{Leaf, Node};
use std::{fs, io, path::Path};

mod exercises;

fn main() {
    println!("\n*** Chapter 10 ***\n");

    countdown(10);

    println!("\n5! = {}\n", factorial(5));

    find_directories(Path::new(".")).unwrap();

    //// Exercises
    println!("\n*** Exercises ***\n");

    exercises::print_every_other(-2, 14);

    println!(
        "\nsum of numbers in a range 1..=10 is {}",
        exercises::sum(1, 10)
    );

    let v = vec![
        Leaf(1),
        Leaf(2),
        Leaf(3),
        Node(vec![Leaf(4), Leaf(5), Leaf(6)]),
        Leaf(7),
        Node(vec![
            Leaf(8),
            Node(vec![
                Leaf(9),
                Leaf(10),
                Leaf(11),
                Node(vec![Leaf(12), Leaf(13), Leaf(14)]),
            ]),
        ]),
        Node(vec![
            Leaf(15),
            Leaf(16),
            Leaf(17),
            Leaf(18),
            Leaf(19),
            Node(vec![
                Leaf(20),
                Leaf(21),
                Leaf(22),
                Node(vec![
                    Leaf(23),
                    Leaf(24),
                    Leaf(25),
                    Node(vec![Leaf(26), Leaf(27), Leaf(28), Leaf(29)]),
                ]),
                Leaf(30),
                Leaf(31),
            ]),
            Leaf(32),
        ]),
        Leaf(33),
    ];
    exercises::print_all_numbers(v);
}

fn countdown(num: u8) {
    println!("Countdown: {num}");
    if num == 0 {
    } else {
        countdown(num - 1);
    }
}

fn factorial(num: u8) -> u8 {
    if num == 0 {
        1
    } else {
        num * factorial(num - 1)
    }
}

#[allow(unused_must_use)]
fn find_directories(dir: &Path) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let path = entry?.path();
            if path.is_dir() {
                println!("path: {:?}", path);
                find_directories(&path);
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(2), 2);
        assert_eq!(factorial(3), 6);
        assert_eq!(factorial(4), 24);
        assert_eq!(factorial(5), 120);
    }
}
