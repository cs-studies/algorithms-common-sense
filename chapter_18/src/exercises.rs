use crate::graph::Neighbor;
use std::collections::{HashSet, VecDeque};
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs() {
        let graph = crate::graph::sample();
        assert!(bfs(&graph.0, &graph.1.borrow().value));
        assert!(!bfs(&graph.0, &"Diana"));
    }
}
