mod doubly_linked_list;
mod linked_list;

use doubly_linked_list::DoublyLinkedList;
use linked_list::{LinkedList, Node};

fn main() {
    println!("\n*** Chapter 14 ***\n");

    let mut n1 = Node::new("once".to_string());
    let mut n2 = Node::new("upon".to_string());
    let mut n3 = Node::new("a".to_string());
    let n4 = Node::new("time".to_string());

    n3.next = n4.into_link();
    n2.next = n3.into_link();
    n1.next = n2.into_link();

    let mut list = LinkedList::new(n1.into_link());
    println!("Read at 1: {:?}", list.read(1));
    println!("Index of 'time': {:?}", list.index_of("time".to_string()));
    println!("Insert 'purple' at 3");
    list.insert(3, "purple".to_string());
    list.delete(0);
    list.delete(3);
    dbg!(&list);

    let mut list = DoublyLinkedList::new();
    list.insert_at_end("once");
    list.insert_at_end("upon");
    list.insert_at_end("a");
    list.insert_at_end("time");
    dbg!(&list);
}
