use crate::graph::{Neighbor, Vertex};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;

pub fn bfs<T>(start: &Neighbor<T>, search_for: &T) -> bool
where
    T: Eq + Hash + Clone,
{
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    visited.insert(start.borrow().value.clone());
    queue.push_back(Rc::clone(start));

    while let Some(current) = queue.pop_front() {
        let current = current.borrow();

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

pub fn shortest_path<T>(start: &Neighbor<T>, end: &T) -> Option<Vec<T>>
where
    T: Debug + Eq + Hash + Clone,
{
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut previous = HashMap::new();

    let start_value = start.borrow().value.clone();
    visited.insert(start_value.clone());
    queue.push_back(Rc::clone(start));

    'outer: while let Some(current) = queue.pop_front() {
        let current = current.borrow();
        for neighbor in &current.neighbors {
            let vertex = neighbor.borrow();
            if visited.insert(vertex.value.clone()) {
                queue.push_back(Rc::clone(neighbor));
                previous.insert(vertex.value.clone(), current.value.clone());
                if vertex.value == *end {
                    break 'outer;
                }
            }
        }
    }

    if !visited.contains(end) {
        return None;
    }

    let mut path = Vec::new();
    let mut current = end;
    while *current != start_value {
        path.push(current.clone());
        current = previous
            .get(current)
            .expect("previous vertex value must be present");
    }
    path.push(start_value);
    path.reverse();
    Some(path)
}

pub(crate) fn sample() -> (
    Neighbor<&'static str>,
    Neighbor<&'static str>,
    Neighbor<&'static str>,
) {
    let idris = Vertex::new("Idris");
    let talia = Vertex::new("Talia");
    let ken = Vertex::new("Ken");
    let marco = Vertex::new("Marco");
    let sasha = Vertex::new("Sasha");
    let lina = Vertex::new("Lina");
    let kamil = Vertex::new("Kamil");

    idris.borrow_mut().add_neighbor(&kamil);
    idris.borrow_mut().add_neighbor(&talia);
    kamil.borrow_mut().add_neighbor(&lina);
    talia.borrow_mut().add_neighbor(&ken);
    ken.borrow_mut().add_neighbor(&marco);
    marco.borrow_mut().add_neighbor(&sasha);
    sasha.borrow_mut().add_neighbor(&lina);

    (idris, kamil, lina)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs() {
        let graph = crate::graph::sample();
        assert!(bfs(&graph.0, &graph.1.borrow().value));
        assert!(!bfs(&graph.0, &"Diana"));
    }

    #[test]
    fn test_shortest_path() {
        let s = sample();
        let s0_value = s.0.borrow().value;
        assert_eq!(shortest_path(&s.0, &"Diana"), None);
        assert_eq!(shortest_path(&s.0, &s0_value), Some(vec![s0_value]));
        assert_eq!(
            shortest_path(&s.0, &s.2.borrow().value),
            Some(vec![s0_value, s.1.borrow().value, s.2.borrow().value]),
        );
    }
}
