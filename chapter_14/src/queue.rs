use std::{cell::Ref, fmt::Debug};
use crate::doubly_linked_list::DoublyLinkedList;

#[derive(Debug)]
pub struct Queue<T> {
    data: DoublyLinkedList<T>,
}

impl<T: Debug> Queue<T> {
    pub fn new() -> Self {
        Self {
            data: DoublyLinkedList::new(),
        }
    }

    pub fn enqueue(&mut self, element: T) {
        self.data.push_end(element)
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.data.pop_front().map(|node| node.data)
    }

    pub fn read(&self) -> Option<Ref<T>> {
        self.data.head.as_ref().map(|node| {
            Ref::map(node.borrow(), |t| &t.data)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue() {
        let mut queue = Queue::new();

        assert!(queue.dequeue().is_none());
        assert!(queue.read().is_none());

        queue.enqueue(1);
        queue.enqueue(2);

        assert_eq!(*queue.read().unwrap(), 1);
        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(*queue.read().unwrap(), 2);
        assert_eq!(queue.dequeue(), Some(2));

        assert!(queue.dequeue().is_none());
        assert!(queue.read().is_none());
    }
}
