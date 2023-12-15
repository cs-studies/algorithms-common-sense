use std::{fs, io, path::Path};

fn main() {
    println!("\n*** Chapter 10 ***\n");

    countdown(10);

    println!("\n5! = {}\n", factorial(5));

    find_directories(Path::new(".")).unwrap();
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

fn find_directories(dir: &Path) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let path = entry?.path();
            if path.is_dir() {
                println!("path: {:?}", path);
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
