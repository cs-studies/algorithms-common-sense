fn main() {
    println!("\n*** Chapter 19 ***\n");

    let words = vec!["tuvi", "leah", "shaya", "rami"];
    let uppercased = make_uppercase(&words);
    println!("{:?}", uppercased);

    let words = vec![
        "tuvi".to_string(),
        "leah".to_string(),
        "shaya".to_string(),
        "rami".to_string(),
    ];
    let uppercased = make_uppercase_2(words);
    println!("{:?}", uppercased);

    let mut a = [1, 4, 5, 2, 9];
    println!("\nArray: {:?}", a);
    println!("Duplicates: {}\n", has_duplicates_3(&mut a));

    let mut a = [65, -55, 45, 45, 10];
    println!("Array: {:?}", a);
    println!("Duplicates: {}\n", has_duplicates_3(&mut a));
}

fn make_uppercase(words: &[&str]) -> Vec<String> {
    let mut result = Vec::with_capacity(words.len());
    for w in words {
        result.push(w.to_uppercase());
    }
    result
}

fn make_uppercase_2(mut words: Vec<String>) -> Vec<String> {
    for w in &mut words {
        // Still, memory is allocated because new String is returned.
        *w = w.to_uppercase();
    }
    words
}

// See /chapter_04/src/main.rs
fn has_duplicates_3(data: &mut [i32]) -> bool {
    data.sort();

    for i in 1..data.len() {
        println!("Checked if {:?}=={:?}", data[i - 1], data[i]);
        if data[i - 1] == data[i] {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_uppercase() {
        assert_eq!(make_uppercase(&[]), Vec::<String>::new());

        let words = ["one", "two", "three"];
        assert_eq!(make_uppercase(&words), vec!["ONE", "TWO", "THREE"]);
    }

    #[test]
    fn test_make_uppercase_2() {
        assert_eq!(make_uppercase_2(vec![]), Vec::<String>::new());

        let words =
            vec!["one".to_string(), "two".to_string(), "three".to_string()];
        assert_eq!(make_uppercase_2(words), vec!["ONE", "TWO", "THREE"]);
    }

    #[test]
    fn test_has_duplicates_3() {
        assert!(!has_duplicates_3(&mut []));
        assert!(!has_duplicates_3(&mut [1]));
        assert!(has_duplicates_3(&mut [1, 1]));
        assert!(!has_duplicates_3(&mut [1, 2]));
        assert!(has_duplicates_3(&mut [1, 5, 3, 9, 1, 4]));
        assert!(!has_duplicates_3(&mut [1, 5, 3, 9, 4]));
    }
}
