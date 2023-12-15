use std::collections::VecDeque;

#[derive(Debug, Default)]
struct Queue<T> {
    data: VecDeque<T>,
}

impl<T> Queue<T> {
    fn enqueue(&mut self, element: T) {
        self.data.push_back(element)
    }

    fn dequeue(&mut self) -> Option<T> {
        self.data.pop_front()
    }

    fn read(&self) -> Option<&T> {
        self.data.front()
    }
}

#[derive(Debug, Default)]
pub struct PrintManager {
    queue: Queue<String>,
}

impl PrintManager {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn queue_job(&mut self, document: String) {
        self.queue.enqueue(document);
    }

    pub fn run(&mut self) {
        while self.queue.read().is_some() {
            Self::print(self.queue.dequeue().unwrap());
        }
    }

    fn print(document: String) -> bool {
        println!("print: {document}");
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue() {
        let mut queue = Queue {
            data: VecDeque::from([10, 40, 30]),
        };
        assert_eq!(queue.data, VecDeque::from([10, 40, 30]));

        queue.enqueue(33);
        assert_eq!(queue.data, VecDeque::from([10, 40, 30, 33]));

        let dequeued = queue.dequeue();
        assert_eq!(dequeued, Some(10));
        assert_eq!(queue.data, VecDeque::from([40, 30, 33]));

        let first = queue.read();
        assert_eq!(first, Some(&40));
    }

    #[test]
    fn test_printer() {
        let mut pm = PrintManager::new();
        pm.queue_job(String::from("First"));
        assert_eq!(pm.queue.read(), Some(&String::from("First")));
    }
}
