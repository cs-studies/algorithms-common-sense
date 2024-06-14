mod linked_list;
use linked_list::{LinkedList, Node};

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
    list.delete(0);
    list.delete(3);
    dbg!(&list);
}
