use std::collections::HashSet;
use std::fmt::{Debug, Formatter, Result};
use std::hash::Hash;
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

    println!("\nTraverse: ");
    alice.borrow().traverse_deep_first(&mut HashSet::new());
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

impl<T: Clone + Debug> Vertex<T> {
    fn traverse_deep_first(&self, visited: &mut HashSet<T>)
    where
        T: Eq + Hash,
    {
        visited.insert(self.value.clone());

        println!("{:?}", &self.value);

        for neighbour in self.neighbours.iter() {
            let vertex = neighbour.borrow();
            if !visited.contains(&vertex.value) {
                vertex.traverse_deep_first(visited);
            }
        }
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
