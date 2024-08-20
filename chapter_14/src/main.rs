mod doubly_linked_list;
mod linked_list;
mod queue;

use doubly_linked_list::DoublyLinkedList;
use linked_list::{LinkedList, Node};
use queue::Queue;

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
    println!("\nRead at 1: {:?}", list.read(1));
    println!("Index of 'time': {:?}", list.index_of("time".to_string()));
    println!("Insert 'purple' at 3");
    list.insert(3, "purple".to_string());
    list.delete(0);
    list.delete(3);
    dbg!(&list);

    let mut list = DoublyLinkedList::new();
    list.push_end("once");
    list.push_end("upon");
    list.push_end("a");
    list.push_end("time");

    dbg!(&list);
    dbg!(list.pop_front());
    dbg!(list.pop_front());
    dbg!(&list);

    let mut q = Queue::new();
    println!("dequeue: {:?}", q.dequeue());
    println!("read: {:?}", q.read());

    q.enqueue("First".to_string());
    q.enqueue("Second".to_string());
    q.enqueue("Third".to_string());

    println!("dequeue: {:?}", q.dequeue());
    println!("read: {:?}", q.read());
    dbg!(&q);

    //// Exercises
    println!("\n*** Exercises ***\n");

    let mut n1 = Node::new("print");
    let mut n2 = Node::new("all");
    let n3 = Node::new("elements");
    n2.next = n3.into_link();
    n1.next = n2.into_link();
    let ll2 = LinkedList::new(n1.into_link());
    println!("print_all:");
    ll2.print_all();

    let mut dl2 = DoublyLinkedList::new();
    dl2.push_end("print");
    dl2.push_end("all");
    dl2.push_end("elements");
    dl2.push_end("reversed");

    println!("print_all:");
    dl2.print_all();
}
