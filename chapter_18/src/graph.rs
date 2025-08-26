use std::cell::RefCell;
use std::collections::{HashSet, VecDeque};
use std::fmt::{Debug, Display, Formatter, Result};
use std::hash::Hash;
use std::rc::Rc;

type Neighbor<T> = Rc<RefCell<Vertex<T>>>;

pub struct Vertex<T> {
    value: T,
    neighbors: Vec<Neighbor<T>>,
}

impl<T> Vertex<T> {
    pub fn new(value: T) -> Neighbor<T> {
        Rc::new(RefCell::new(Self {
            value,
            neighbors: Vec::new(),
        }))
    }

    pub fn add_neighbor(&mut self, neighbor: &Neighbor<T>) {
        self.neighbors.push(Rc::clone(neighbor));
    }
}

pub mod depth_first {
    use super::*;

    pub fn traverse<T>(start: &Neighbor<T>)
    where
        T: Display + Eq + Hash + Clone,
    {
        fn inner<T>(start: &Neighbor<T>, visited: &mut HashSet<T>)
        where
            T: Display + Eq + Hash + Clone,
        {
            let vertex = start.borrow();
            if !visited.insert(vertex.value.clone()) {
                return;
            }
            println!("{}", vertex.value);

            for neighbor in &vertex.neighbors {
                inner(neighbor, visited);
            }
        }
        inner(start, &mut HashSet::new());
    }

    pub fn search<T>(start: &Neighbor<T>, search_for: &T) -> bool
    where
        T: Eq + Hash + Clone,
    {
        fn inner<T>(
            start: &Neighbor<T>,
            search_for: &T,
            visited: &mut HashSet<T>,
        ) -> bool
        where
            T: Eq + Hash + Clone,
        {
            let vertex = start.borrow();
            if *search_for == vertex.value {
                return true;
            }
            if !visited.insert(vertex.value.clone()) {
                return false;
            }
            for neighbor in &vertex.neighbors {
                if inner(neighbor, search_for, visited) {
                    return true;
                }
            }
            false
        }
        inner(start, search_for, &mut HashSet::new())
    }
}

pub mod breadth_first {
    use super::*;

    pub fn traverse<T>(start: &Neighbor<T>)
    where
        T: Display + Eq + Hash + Clone,
    {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        visited.insert(start.borrow().value.clone());
        queue.push_back(Rc::clone(start));

        while let Some(current) = queue.pop_front() {
            let current = current.borrow();
            println!("{}", current.value);

            for neighbor in &current.neighbors {
                let vertex = neighbor.borrow();
                if visited.insert(vertex.value.clone()) {
                    queue.push_back(Rc::clone(neighbor));
                }
            }
        }
    }

    pub fn search<T>(start: &Neighbor<T>, search_for: &T) -> bool
    where
        T: Eq + Hash + Clone,
    {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        visited.insert(start.borrow().value.clone());
        queue.push_back(Rc::clone(start));

        while let Some(current_rc) = queue.pop_front() {
            let current = current_rc.borrow();

            if *search_for == current.value {
                return true;
            }

            for neighbor in &current.neighbors {
                let vertex = neighbor.borrow();
                if visited.insert(vertex.value.clone()) {
                    queue.push_back(Rc::clone(neighbor));
                }
            }
        }

        false
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_depth_first() {
        let alice = sample_graph();
        assert!(depth_first::search(&alice, &"Bob",));
        assert!(!depth_first::search(&alice, &"Diana"));
    }

    #[test]
    fn test_search_breadth_first() {
        let alice = sample_graph();
        assert!(breadth_first::search(&alice, &"Bob"));
        assert!(!breadth_first::search(&alice, &"Diana"));
    }

    fn sample_graph() -> Neighbor<&'static str> {
        let alice = Vertex::new("Alice");
        let bob = Vertex::new("Bob");
        let cynthia = Vertex::new("Cynthia");

        alice.borrow_mut().add_neighbor(&bob);
        alice.borrow_mut().add_neighbor(&cynthia);
        bob.borrow_mut().add_neighbor(&cynthia);
        cynthia.borrow_mut().add_neighbor(&bob);

        alice
    }
}
