use std::cmp::PartialEq;
use std::fmt::Debug;

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Link<T>,
}

#[derive(Debug)]
pub struct Node<T> {
    data: T,
    pub next: Link<T>,
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
        let mut new_node = Node::new(value);
        for _ in 0..at {
            match link {
                Some(ref mut node) => link = &mut node.next,
                None => panic!("Index out of bounds"),
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
        let mut previous: Link<T> = None;
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

impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        Self { data, next: None }
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
        let mut node = Node::new(11);
        assert_eq!(node.data, 11);
        assert!(node.next.is_none());
        node.next = Node::new(22).into_link();
        assert!(node.next.is_some());
    }

    #[test]
    fn test_list_empty() {
        let empty = LinkedList::<i32>::new(None);
        assert!(empty.head.is_none());
    }

    #[test]
    fn test_list_non_empty() {
        let node = Node::new(11);
        let list = LinkedList::new(node.into_link());
        assert!(list.head.is_some());
    }

    #[test]
    fn test_list_read() {
        let mut node1 = Node::new(11);
        node1.next = Node::new(22).into_link();
        let list = LinkedList::new(node1.into_link());
        assert_eq!(list.read(0), Some(&11));
        assert_eq!(list.read(1), Some(&22));
    }

    #[test]
    fn test_list_index_of() {
        let mut node1 = Node::new(11);
        node1.next = Node::new(22).into_link();
        let list = LinkedList::new(node1.into_link());
        assert_eq!(list.index_of(11), Some(0));
        assert_eq!(list.index_of(22), Some(1));
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
    #[should_panic(expected = "Index out of bounds")]
    fn test_list_insert_panics() {
        let mut list = LinkedList::<i32>::new(None);
        list.insert(1, 22);
    }

    #[test]
    fn test_list_delete() {
        let mut list = LinkedList::<i32>::new(None);
        assert!(list.head.is_none());
        list.delete(0);
        assert!(list.head.is_none());

        let mut node1 = Node::new(11);
        node1.next = Node::new(22).into_link();
        let mut list = LinkedList::new(node1.into_link());
        list.delete(0);
        assert_eq!(list.index_of(11), None);
        assert_eq!(list.index_of(22), Some(0));

        let mut node1 = Node::new(111);
        node1.next = Node::new(22).into_link();
        list = LinkedList::new(node1.into_link());
        list.delete(1);
        assert_eq!(list.index_of(111), Some(0));
        assert_eq!(list.index_of(222), None);

        list.delete(10);
    }

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
