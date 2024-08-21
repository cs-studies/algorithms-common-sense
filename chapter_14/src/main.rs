mod doubly_linked_list;
mod linked_list;
mod queue;

use doubly_linked_list::DoublyLinkedList;
use linked_list::{LinkedList, Node};
use queue::Queue;

fn main() {
    println!("\n*** Chapter 14 ***\n");

    let mut n1 = Node::new("once");
    let mut n2 = Node::new("upon");
    let mut n3 = Node::new("a");
    let n4 = Node::new("time");

    n3.next = n4.into_link();
    n2.next = n3.into_link();
    n1.next = n2.into_link();

    let mut ll = LinkedList::new(n1.into_link());
    println!("Read at 1: {:?}", ll.read(1));
    println!("Index of 'time': {:?}", ll.index_of("time"));
    println!("Insert 'purple' at 3");
    ll.insert(3, "purple");
    ll.delete(0);
    ll.delete(3);
    dbg!(&ll);

    let mut dl = DoublyLinkedList::new();
    dl.push_end("once");
    dl.push_end("upon");
    dl.push_end("a");
    dl.push_end("time");

    dbg!(&dl);
    dbg!(dl.pop_front());
    dbg!(dl.pop_front());
    dbg!(&dl);

    let mut q = Queue::new();
    println!("dequeue: {:?}", q.dequeue());
    println!("read: {:?}", q.read());

    q.enqueue("First");
    q.enqueue("Second");
    q.enqueue("Third");

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

    println!("read_last: {:?}", ll2.read_last());
}
