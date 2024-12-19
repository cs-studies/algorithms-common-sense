use std::cell::RefCell;
use std::collections::HashSet;
use std::fmt::{Debug, Display, Formatter, Result};
use std::hash::Hash;
use std::rc::Rc;

type Neighbour<T> = Rc<RefCell<Vertex<T>>>;

fn main() {
    println!("\n*** Chapter 18 ***\n");

    let alice = Vertex::new("Alice");
    let bob = Vertex::new("Bob");
    let cynthia = Vertex::new("Cynthia");

    alice.borrow_mut().add_neighbour(Rc::clone(&bob));
    alice.borrow_mut().add_neighbour(Rc::clone(&cynthia));

    bob.borrow_mut().add_neighbour(Rc::clone(&cynthia));
    cynthia.borrow_mut().add_neighbour(Rc::clone(&bob));

    dbg!(&alice);
    dbg!(&bob);
    dbg!(&cynthia);

    println!("\nTraverse: ");
    alice.borrow().traverse_deep_first(&mut HashSet::new());

    println!(
        "Found Bob: {}",
        alice
            .borrow()
            .search_deep_first(&"Bob", &mut HashSet::new())
    );
    println!(
        "Found Alice: {}",
        bob.borrow()
            .search_deep_first(&"Alice", &mut HashSet::new())
    );
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

    fn add_neighbour(&mut self, neighbour: Neighbour<T>) {
        self.neighbours.push(neighbour);
    }
}

impl<T: Clone + Eq + Hash> Vertex<T> {
    fn traverse_deep_first(&self, visited: &mut HashSet<T>)
    where
        T: Display,
    {
        visited.insert(self.value.clone());

        println!("{}", &self.value);

        for neighbour in self.neighbours.iter() {
            let vertex = neighbour.borrow();
            if !visited.contains(&vertex.value) {
                vertex.traverse_deep_first(visited);
            }
        }
    }

    fn search_deep_first(
        &self,
        search_for: &T,
        visited: &mut HashSet<T>,
    ) -> bool {
        if search_for == &self.value {
            return true;
        }
        visited.insert(self.value.clone());

        for neighbour in self.neighbours.iter() {
            let vertex = neighbour.borrow();
            if !visited.contains(&vertex.value)
                && vertex.search_deep_first(search_for, visited)
            {
                return true;
            }
        }

        false
    }
}

impl<T: Debug + Clone> Debug for Vertex<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let neighbours: Vec<_> = self
            .neighbours
            .iter()
            .map(|n| n.borrow().value.clone())
            .collect();

        f.debug_struct("Vertex")
            .field("value", &self.value)
            .field("neighbours", &neighbours)
            .finish()
    }
}
