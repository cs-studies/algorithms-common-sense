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
}
