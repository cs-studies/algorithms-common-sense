use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

type Link<T> = Rc<RefCell<Node<T>>>;

#[derive(Debug)]
pub struct DoublyLinkedList<T> {
    head: Option<Link<T>>,
    tail: Option<Link<T>>,
}

pub struct Node<T> {
    data: T,
    prev: Option<Link<T>>,
    next: Option<Link<T>>,
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    pub fn push_end(&mut self, value: T) {
        let link = Node::new(value);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(Rc::clone(&link));
                link.borrow_mut().prev = Some(old_tail);
            }
            None => {
                self.head = Some(Rc::clone(&link));
            }
        }
        self.tail = Some(link);
    }
    pub fn pop_front(&mut self) -> Option<Node<T>> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(next) => {
                    next.borrow_mut().prev = None;
                    self.head = Some(Rc::clone(&next));
                }
                None => {
                    self.tail = None;
                }
            }
            Rc::try_unwrap(old_head)
                .ok()
                .expect("RefCell contained in Rc")
                .into_inner()
        })
    }
}

impl<T> Node<T> {
    pub fn new(data: T) -> Link<T> {
        Rc::new(RefCell::new(Self {
            data,
            prev: None,
            next: None,
        }))
    }
}

impl<T: fmt::Debug> fmt::Debug for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Node")
            .field("data", &self.data)
            .field("prev.is_some()", &self.prev.is_some())
            .field("next", &self.next)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_new() {
        let node = Node::new(11);
        assert_eq!(node.borrow().data, 11);
        assert!(node.borrow().next.is_none());
        assert!(node.borrow().prev.is_none());
    }

    #[test]
    fn test_list_empty() {
        let empty = DoublyLinkedList::<i32>::new();
        assert!(empty.head.is_none());
        assert!(empty.tail.is_none());
    }

    #[test]
    fn test_list_push_end() {
        let mut list = DoublyLinkedList::<i32>::new();
        list.push_end(11);

        let head = list.head.as_ref().unwrap();
        assert_eq!(head.borrow().data, 11);
        assert!(head.borrow().prev.is_none());
        assert!(head.borrow().next.is_none());

        let tail = list.tail.as_ref().unwrap();
        assert_eq!(tail.borrow().data, 11);
        assert!(tail.borrow().prev.is_none());
        assert!(tail.borrow().next.is_none());

        list.push_end(22);
        let head = list.head.as_ref().unwrap();
        assert_eq!(head.borrow().data, 11);
        assert!(head.borrow().prev.is_none());
        assert!(head.borrow().next.is_some());

        let tail = list.tail.as_ref().unwrap();
        assert_eq!(tail.borrow().data, 22);
        assert!(tail.borrow().prev.is_some());
        assert!(tail.borrow().next.is_none());
    }

    #[test]
    fn test_list_pop_front() {
        let mut list = DoublyLinkedList::<i32>::new();
        assert!(list.pop_front().is_none());

        list.push_end(13);
        list.push_end(14);

        let node = list.pop_front().expect("the node 13 is in place");
        assert_eq!(node.data, 13);

        let head = list.head.as_ref().unwrap();
        assert_eq!(head.borrow().data, 14);
        assert!(head.borrow().prev.is_none());
        assert!(head.borrow().next.is_none());

        let tail = list.tail.as_ref().unwrap();
        assert_eq!(tail.borrow().data, 14);
        assert!(tail.borrow().prev.is_none());
        assert!(tail.borrow().next.is_none());

        let node = list.pop_front().expect("the node 14 is in place");
        assert_eq!(node.data, 14);

        assert!(list.pop_front().is_none());
    }
}
