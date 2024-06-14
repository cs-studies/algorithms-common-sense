use std::cmp::PartialEq;
use std::fmt::Debug;

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Link<T>,
}

impl<T: Debug + PartialEq> LinkedList<T> {
    pub fn new(head: Link<T>) -> Self {
        Self { head }
    }

    pub fn read(&self, at: usize) -> Option<&T> {
        let mut link = &self.head;
        for _ in 0..at {
            match link {
                Some(node) => link = &node.next,
                None => return None,
            }
        }
        link.as_ref().map(|node| &node.data)
    }

    pub fn index_of(&self, value: T) -> Option<usize> {
        let mut link = &self.head;
        let mut i = 0;
        while let Some(node) = link {
            if node.data == value {
                return Some(i);
            }
            link = &node.next;
            i += 1;
        }
        None
    }

    pub fn insert(&mut self, at: usize, value: T) {
        let mut link = &mut self.head;
        let mut new_node = Node::new(value, None);
        for _ in 0..at {
            match link {
                Some(ref mut node) => link = &mut node.next,
                None => return,
            }
        }
        new_node.next = link.take();
        *link = new_node.into_link();
    }

    pub fn delete(&mut self, at: usize) {
        let mut link = &mut self.head;
        for _ in 0..at {
            match link {
                Some(ref mut node) => link = &mut node.next,
                None => return,
            }
        }
        if let Some(node) = link {
            *link = node.next.take();
        }
    }
}

#[derive(Debug)]
pub struct Node<T> {
    data: T,
    next: Link<T>,
}

impl<T> Node<T> {
    pub fn new(data: T, next: Link<T>) -> Self {
        Self { data, next }
    }

    pub fn into_link(self) -> Link<T> {
        Some(Box::new(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_new() {
        let node = Node::new(555, None);
        assert_eq!(node.data, 555);
        assert!(node.next.is_none());
    }

    #[test]
    fn test_list_empty() {
        let empty = LinkedList::<i32>::new(None);
        assert!(empty.head.is_none());
    }

    #[test]
    fn test_list_non_empty() {
        let node = Node::new(555, None);
        let list = LinkedList::new(node.into_link());
        assert!(list.head.is_some());
    }

    #[test]
    fn test_list_read() {
        let node2 = Node::new(222, None);
        let node1 = Node::new(111, node2.into_link());
        let list = LinkedList::new(node1.into_link());
        assert_eq!(list.read(0), Some(&111));
        assert_eq!(list.read(1), Some(&222));
    }

    #[test]
    fn test_list_index_of() {
        let node2 = Node::new(222, None);
        let node1 = Node::new(111, node2.into_link());
        let list = LinkedList::new(node1.into_link());
        assert_eq!(list.index_of(111), Some(0));
        assert_eq!(list.index_of(222), Some(1));
        assert!(list.index_of(55).is_none());
    }

    #[test]
    fn test_list_insert() {
        let mut list = LinkedList::<i32>::new(None);
        assert!(list.index_of(11).is_none());
        assert!(list.index_of(22).is_none());

        list.insert(0, 11);
        assert_eq!(list.index_of(11), Some(0));

        list.insert(1, 22);
        assert_eq!(list.index_of(22), Some(1));
    }

    #[test]
    fn test_list_delete() {
        let mut list = LinkedList::<i32>::new(None);
        assert!(list.head.is_none());
        list.delete(0);
        assert!(list.head.is_none());

        let node2 = Node::new(222, None);
        let node1 = Node::new(111, node2.into_link());
        let mut list = LinkedList::new(node1.into_link());
        list.delete(0);
        assert_eq!(list.index_of(111), None);
        assert_eq!(list.index_of(222), Some(0));

        let node2 = Node::new(222, None);
        let node1 = Node::new(111, node2.into_link());
        list = LinkedList::new(node1.into_link());
        list.delete(1);
        assert_eq!(list.index_of(111), Some(0));
        assert_eq!(list.index_of(222), None);

        list.delete(10);
    }
}
