pub fn is_palindrome(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();

    for i in 0..(len / 2) {
        if chars[i] != chars[len - 1 - i] {
            return false;
        }
    }
    true
}

//// Rust Extras
#[allow(dead_code)]
fn is_palindrome_extra_1(s: &str) -> bool {
    let mid = s.len() / 2;
    let left = s.chars().take(mid);
    let right = s.chars().rev().take(mid);
    for (l, r) in left.zip(right) {
        if l != r {
            return false;
        }
    }
    true
}

#[allow(dead_code)]
fn is_palindrome_extra_2(s: &str) -> bool {
    let mid = s.len() / 2;
    let left = s.chars().take(mid);
    let right = s.chars().rev().take(mid);
    left.eq(right)
}

#[allow(dead_code)]
fn is_palindrome_extra_3(s: &str) -> bool {
    s == s.chars().rev().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome(""));
        assert!(is_palindrome("a"));
        assert!(is_palindrome("abba"));
        assert!(!is_palindrome("abja"));
        assert!(!is_palindrome("rotör"));
        assert!(is_palindrome("rötör"));
        assert!(is_palindrome("racecar"));
    }

    #[test]
    fn test_is_palindrome_extra_1() {
        assert!(is_palindrome_extra_1(""));
        assert!(is_palindrome_extra_1("a"));
        assert!(is_palindrome_extra_1("abba"));
        assert!(!is_palindrome_extra_1("abja"));
        assert!(!is_palindrome_extra_1("rotör"));
        assert!(is_palindrome_extra_1("rötör"));
        assert!(is_palindrome_extra_1("racecar"));
    }

    #[test]
    fn test_is_palindrome_extra_2() {
        assert!(is_palindrome_extra_2(""));
        assert!(is_palindrome_extra_2("a"));
        assert!(is_palindrome_extra_2("abba"));
        assert!(!is_palindrome_extra_2("abja"));
        assert!(!is_palindrome_extra_2("rotör"));
        assert!(is_palindrome_extra_2("rötör"));
        assert!(is_palindrome_extra_2("racecar"));
    }

    #[test]
    fn test_is_palindrome_extra_3() {
        assert!(is_palindrome_extra_3(""));
        assert!(is_palindrome_extra_3("a"));
        assert!(is_palindrome_extra_3("abba"));
        assert!(!is_palindrome_extra_3("abja"));
        assert!(!is_palindrome_extra_3("rotör"));
        assert!(is_palindrome_extra_3("rötör"));
        assert!(is_palindrome_extra_3("racecar"));
    }
}
