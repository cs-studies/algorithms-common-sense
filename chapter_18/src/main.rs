mod weighted_graph;

use std::cell::RefCell;
use std::collections::{HashSet, VecDeque};
use std::fmt::{Debug, Display, Formatter, Result};
use std::hash::Hash;
use std::rc::Rc;
use weighted_graph::WeightedVertex;

type Neighbor<T> = Rc<RefCell<Vertex<T>>>;

fn main() {
    println!("\n*** Chapter 18 ***\n");

    let alice = Vertex::new("Alice");
    let bob = Vertex::new("Bob");
    let cynthia = Vertex::new("Cynthia");

    alice.borrow_mut().add_neighbor(Rc::clone(&bob));
    alice.borrow_mut().add_neighbor(Rc::clone(&cynthia));

    bob.borrow_mut().add_neighbor(Rc::clone(&cynthia));
    cynthia.borrow_mut().add_neighbor(Rc::clone(&bob));

    dbg!(&alice);
    dbg!(&bob);
    dbg!(&cynthia);

    println!("\nTraverse deapth-first: ");
    alice.borrow().traverse_depth_first(&mut HashSet::new());

    println!(
        "\nFound Bob: {}",
        alice
            .borrow()
            .search_depth_first(&"Bob", &mut HashSet::new())
    );
    println!(
        "Found Alice: {}",
        bob.borrow()
            .search_depth_first(&"Alice", &mut HashSet::new())
    );

    println!("\nTraverse breadth-first: ");
    alice.borrow().traverse_breadth_first(&alice);

    let mut dallas = WeightedVertex::new("Dallas");
    let mut toronto = WeightedVertex::new("Toronto");

    dallas.add_neighbor(toronto.value, 138);
    toronto.add_neighbor(dallas.value, 216);

    println!("\n{:?}", dallas);
    println!("{:?}", toronto);
}

struct Vertex<T> {
    value: T,
    neighbors: Vec<Neighbor<T>>,
}

impl<T> Vertex<T> {
    fn new(value: T) -> Neighbor<T> {
        Rc::new(RefCell::new(Self {
            value,
            neighbors: Vec::new(),
        }))
    }

    fn add_neighbor(&mut self, neighbor: Neighbor<T>) {
        self.neighbors.push(neighbor);
    }
}

impl<T: Clone + Eq + Hash> Vertex<T> {
    fn traverse_depth_first(&self, visited: &mut HashSet<T>)
    where
        T: Display,
    {
        visited.insert(self.value.clone());

        println!("{}", &self.value);

        for neighbor in self.neighbors.iter() {
            let vertex = neighbor.borrow();
            if !visited.contains(&vertex.value) {
                vertex.traverse_depth_first(visited);
            }
        }
    }

    fn search_depth_first(
        &self,
        search_for: &T,
        visited: &mut HashSet<T>,
    ) -> bool {
        if search_for == &self.value {
            return true;
        }
        visited.insert(self.value.clone());

        for neighbor in self.neighbors.iter() {
            let vertex = neighbor.borrow();
            if !visited.contains(&vertex.value)
                && vertex.search_depth_first(search_for, visited)
            {
                return true;
            }
        }

        false
    }

    fn traverse_breadth_first(&self, self_rc: &Neighbor<T>)
    where
        T: Display,
    {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        visited.insert(self.value.clone());
        queue.push_back(Rc::clone(self_rc));

        while let Some(current_rc) = queue.pop_front() {
            let current = current_rc.borrow();
            println!("{}", current.value);

            for neighbor in &current.neighbors {
                let vertex = neighbor.borrow();
                if visited.insert(vertex.value.clone()) {
                    queue.push_back(Rc::clone(neighbor));
                }
            }
        }
    }
}

impl<T: Debug + Clone> Debug for Vertex<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let neighbors: Vec<_> = self
            .neighbors
            .iter()
            .map(|n| n.borrow().value.clone())
            .collect();

        f.debug_struct("Vertex")
            .field("value", &self.value)
            .field("neighbors", &neighbors)
            .finish()
    }
}
