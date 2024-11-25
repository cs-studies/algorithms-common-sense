#![allow(unused_variables, dead_code, unused_imports)]

use std::fmt::{Debug, Formatter, Result};
use std::{cell::RefCell, rc::Rc};

type Neighbour<T> = Rc<RefCell<Vertex<T>>>;

fn main() {
    println!("\n*** Chapter 18 ***\n");

    let alice = Vertex::new("Alice");
    let bob = Vertex::new("Bob");
    let cynthia = Vertex::new("Cynthia");

    alice.borrow_mut().add_neigbour(Rc::clone(&bob));
    alice.borrow_mut().add_neigbour(Rc::clone(&cynthia));

    bob.borrow_mut().add_neigbour(Rc::clone(&cynthia));
    cynthia.borrow_mut().add_neigbour(Rc::clone(&bob));

    dbg!(&alice);
    dbg!(&bob);
    dbg!(&cynthia);
}

struct Vertex<T> {
    value: T,
    neighbours: Vec<Neighbour<T>>,
}

impl<T> Vertex<T> {
    fn new(value: T) -> Neighbour<T> {
        Rc::new(RefCell::new(Self {
            value,
            neighbours: Vec::new(),
        }))
    }

    fn add_neigbour(&mut self, neighbour: Neighbour<T>) {
        self.neighbours.push(neighbour);
    }
}

impl<T: Debug + Clone> Debug for Vertex<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let neigbours: Vec<_> = self
            .neighbours
            .iter()
            .map(|n| n.borrow().value.clone())
            .collect();

        f.debug_struct("Vertex")
            .field("value", &self.value)
            .field("neigbours", &neigbours)
            .finish()
    }
}
