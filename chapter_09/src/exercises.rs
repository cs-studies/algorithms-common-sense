pub fn reverse(line: &str) -> String {
    let mut stack: Stack<char> = Stack::default();
    for ch in line.chars() {
        stack.push(ch);
    }
    let mut result = String::from("");
    while let Some(popped) = stack.pop() {
        result.push(popped);
    }
    result
}

#[derive(Default)]
struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    fn push(&mut self, element: T) {
        self.data.push(element);
    }

    fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse() {
        assert_eq!(reverse("Frogs"), "sgorF".to_string());
    }
}
