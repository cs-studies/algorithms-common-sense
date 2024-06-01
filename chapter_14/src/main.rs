fn main() {
    println!("\n*** Chapter 14 ***\n");

    let n4 = Node::new("time".to_string(), None);
    let n3 = Node::new("a".to_string(), n4.into_link());
    let n2 = Node::new("upon".to_string(), n3.into_link());
    let n1 = Node::new("once".to_string(), n2.into_link());

    let list = LinkedList::new(n1.into_link());
    let read0 = list.read(2);
    println!("{:?}", read0);
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct LinkedList<T> {
    head: Link<T>,
}

impl<T: std::fmt::Debug> LinkedList<T> {
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
}

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Link<T>,
}

impl<T> Node<T> {
    fn new(data: T, next: Option<Box<Self>>) -> Self {
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
}
