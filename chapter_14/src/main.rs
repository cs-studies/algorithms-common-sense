use std::cmp::PartialEq;
use std::fmt::Debug;

fn main() {
    println!("\n*** Chapter 14 ***\n");

    let n4 = Node::new("time".to_string(), None);
    let n3 = Node::new("a".to_string(), n4.into_link());
    let n2 = Node::new("upon".to_string(), n3.into_link());
    let n1 = Node::new("once".to_string(), n2.into_link());

    let mut list = LinkedList::new(n1.into_link());
    println!("Read at 1: {:?}", list.read(1));
    println!("Index of 'time': {:?}", list.index_of("time".to_string()));
    println!("Insert 'purple' at 3");
    list.insert(3, "purple".to_string());
    dbg!(list);
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct LinkedList<T> {
    head: Link<T>,
}

impl<T: Debug + PartialEq> LinkedList<T> {
    fn new(head: Link<T>) -> Self {
        Self { head }
    }

    fn read(&self, at: usize) -> Option<&T> {
        let mut current_link = &self.head;
        for _ in 0..at {
            match current_link {
                Some(node) => current_link = &node.next,
                None => return None,
            }
        }
        current_link.as_ref().map(|node| &node.data)
    }

    fn index_of(&self, value: T) -> Option<usize> {
        let mut current_link = &self.head;
        let mut i = 0;
        while let Some(node) = current_link {
            if node.data == value {
                return Some(i);
            }
            current_link = &node.next;
            i += 1;
        }
        None
    }

    fn insert(&mut self, at: usize, value: T) {
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
}

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Link<T>,
}

impl<T> Node<T> {
    fn new(data: T, next: Link<T>) -> Self {
        Self { data, next }
    }

    fn into_link(self) -> Link<T> {
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
    fn test_list_insert_at() {
        let mut list = LinkedList::<i32>::new(None);
        assert!(list.index_of(11).is_none());
        assert!(list.index_of(22).is_none());

        list.insert(0, 11);
        assert_eq!(list.index_of(11), Some(0));

        list.insert(1, 22);
        assert_eq!(list.index_of(22), Some(1));
    }
}
