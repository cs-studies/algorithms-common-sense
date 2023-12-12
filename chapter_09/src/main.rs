//! Starting this chapter, we use more of Rust-specific syntax constructs, such
//! as generics, derive attribute, etc.

use std::collections::HashMap;

fn main() {
    println!("\n*** Chapter 09 ***\n");

    let mut stack = Stack::new(vec![10, 40, 30]);
    println!("{:?}", &stack);
    stack.push(33);
    println!("after push: {:?}", &stack);
    println!("popped: {:?}", stack.pop());
    println!("{:?}", &stack);
    println!("last: {:?}", stack.read());
    println!("{:?}\n", &stack);

    let code = "( var x = { y: [1, 2, 3] } )";
    println!("Lint: {}", code);
    println!("Result: {:?}\n", Linter::new().lint(code));

    let code = "( var x = { y: [1, 2, 3 })";
    println!("Lint: {}", &code);
    println!("Result: {:?}", Linter::new().lint(code));
}

#[derive(Debug)]
struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    fn new(data: Vec<T>) -> Self {
        Self { data }
    }

    fn push(&mut self, element: T) {
        self.data.push(element);
    }

    fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    fn read(&self) -> Option<&T> {
        self.data.last()
    }
}

#[derive(Debug)]
struct Linter {
    stack: Stack<char>,
    braces: HashMap<char, char>,
}

impl Linter {
    fn new() -> Self {
        Self {
            stack: Stack::new(Vec::<char>::new()),
            braces: HashMap::from([('(', ')'), ('[', ']'), ('{', '}')]),
        }
    }

    fn is_opening_brace(&self, ch: char) -> bool {
        self.braces.contains_key(&ch)
    }

    fn is_closing_brace(&self, ch: char) -> bool {
        self.braces.values().any(|&x| x == ch)
    }

    fn is_match(&self, opening_brace: char, closing_brace: char) -> bool {
        *self.braces.get(&opening_brace).unwrap() == closing_brace
    }

    fn lint(&mut self, text: &str) -> Result<bool, String> {
        for ch in text.chars() {
            if self.is_opening_brace(ch) {
                self.stack.push(ch);
            } else if self.is_closing_brace(ch) {
                if let Some(popped_brace) = self.stack.pop() {
                    if !self.is_match(popped_brace, ch) {
                        return Err(format!("'{ch}' has mismatched opening brace"));
                    }
                } else {
                    return Err(format!("'{ch}' does not have opening brace"));
                }
            }
        }

        if let Some(last) = self.stack.read() {
            return Err(format!("'{last}' does not have closing brace"));
        }

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lint() {
        assert_eq!(
            Linter::new().lint("(var x = 2").unwrap_err(),
            String::from("'(' does not have closing brace")
        );
        assert_eq!(
            Linter::new().lint("var x = 2;)").unwrap_err(),
            String::from("')' does not have opening brace")
        );
        assert_eq!(
            Linter::new().lint("(var x = [1, 2, 3)]").unwrap_err(),
            String::from("')' has mismatched opening brace")
        );
        assert!(Linter::new().lint("( var x = { y: [1, 2, 3] } )").is_ok());
    }
}
