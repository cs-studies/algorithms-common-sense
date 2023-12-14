use std::collections::HashMap;

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
pub struct Linter {
    stack: Stack<char>,
    braces: HashMap<char, char>,
}

impl Linter {
    pub fn new() -> Self {
        Self {
            stack: Stack::new(Vec::<char>::new()),
            braces: HashMap::from([('(', ')'), ('[', ']'), ('{', '}')]),
        }
    }

    pub fn lint(&mut self, text: &str) -> Result<bool, String> {
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

    fn is_opening_brace(&self, ch: char) -> bool {
        self.braces.contains_key(&ch)
    }

    fn is_closing_brace(&self, ch: char) -> bool {
        self.braces.values().any(|&x| x == ch)
    }

    fn is_match(&self, opening_brace: char, closing_brace: char) -> bool {
        *self.braces.get(&opening_brace).unwrap() == closing_brace
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack() {
        let mut stack = Stack::new(vec![10, 40, 30]);
        assert_eq!(stack.data, vec![10, 40, 30]);

        stack.push(33);
        assert_eq!(stack.data, vec![10, 40, 30, 33]);

        let popped = stack.pop();
        assert_eq!(popped, Some(33));
        assert_eq!(stack.data, vec![10, 40, 30]);

        let last = stack.read();
        assert_eq!(last, Some(&30));
    }

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
