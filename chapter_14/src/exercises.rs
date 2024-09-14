use crate::{
    doubly_linked_list::DoublyLinkedList,
    linked_list::LinkedList,
};
use std::{fmt::Debug, rc::Rc};

impl<T: Debug> LinkedList<T> {
    pub fn print_all(&self) {
        let mut link = &self.head;
        while let Some(node) = link {
            println!("{:?}", node.data);
            link = &node.next;
        }
    }

    pub fn read_last(&self) -> Option<&T> {
        let mut link = &self.head;
        while let Some(node) = link {
            if node.next.is_none() {
                return Some(&node.data);
            }
            link = &node.next;
        }
        None
    }

    pub fn reverse(&mut self) {
        let mut previous = None;
        let mut current = self.head.take();

        while let Some(mut current_node) = current {
            let next = current_node.next.take();
            current_node.next = previous;
            previous = Some(current_node);
            current = next;
        }

        self.head = previous;
    }
}

impl<T: Debug> DoublyLinkedList<T> {
    pub fn print_all(&self) {
        let mut current = self.tail.as_ref().map(Rc::clone);
        while let Some(node) = current {
            println!("{:?}", node.borrow().data);
            current = node.borrow().prev.as_ref().map(Rc::clone);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_last() {
        let mut list = LinkedList::<char>::new(None);
        assert!(list.read_last().is_none());

        list.insert(0, 'A');
        assert_eq!(list.read_last().unwrap(), &'A');

        list.insert(1, 'B');
        assert_eq!(list.read_last().unwrap(), &'B');
    }

    #[test]
    fn test_reverse() {
        let mut list = LinkedList::<char>::new(None);

        list.insert(0, 'a');
        list.insert(1, 'b');
        list.insert(2, 'c');
        list.insert(3, 'd');

        list.reverse();

        assert_eq!(list.read(0), Some(&'d'));
        assert_eq!(list.read(1), Some(&'c'));
        assert_eq!(list.read(2), Some(&'b'));
        assert_eq!(list.read(3), Some(&'a'));
    }
}
