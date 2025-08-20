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

    pub fn add_neighbor(&mut self, neighbor: Neighbor<T>) {
        self.neighbors.push(neighbor);
    }
}

impl<T: Clone + Eq + Hash> Vertex<T> {
    pub fn traverse_depth_first(&self, visited: &mut HashSet<T>)
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

    pub fn search_depth_first(
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

    pub fn traverse_breadth_first(&self, self_rc: &Neighbor<T>)
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
