use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

use crate::dijkstra::{Name, Price, Route};

pub fn shortest_path(
    routes: &HashMap<Name, Route>,
    start: Name,
    destination: Name,
) -> Vec<Name> {
    let mut prices: HashMap<Name, Price> = HashMap::new();
    let mut stopovers: HashMap<Name, Name> = HashMap::new();
    let mut visited: HashSet<Name> = HashSet::new();
    let mut unvisited: BinaryHeap<Reverse<(Price, Name)>> = BinaryHeap::new();

    prices.insert(start, 0);
    unvisited.push(Reverse((0, start)));

    while let Some(Reverse((price, current))) = unvisited.pop() {
        if !visited.insert(current) {
            continue;
        }

        if let Some(neighbors) = routes.get(&current) {
            for (&adjacent, &adj_price) in neighbors {
                let new_price = price.saturating_add(adj_price);
                if prices.get(&adjacent).is_none_or(|&p| new_price < p) {
                    prices.insert(adjacent, new_price);
                    stopovers.insert(adjacent, current);
                    unvisited.push(Reverse((new_price, adjacent)));
                }
            }
        }
    }

    let mut path = Vec::new();
    let mut current = destination;

    while current != start {
        path.push(current);
        current = stopovers
            .get(&current)
            .expect("stopovers must contain cities unless start is reached");
    }

    path.push(start);
    path.reverse();
    path
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dijkstra::sample;

    #[test]
    fn test_shortest_path() {
        let s = sample::data();
        let path = shortest_path(&s.routes, s.atlanta.name, s.elpaso.name);
        assert_eq!(
            path,
            vec![s.atlanta.name, s.denver.name, s.chicago.name, s.elpaso.name]
        );
    }

    #[test]
    fn test_shortest_path_same_city() {
        let s = sample::data();
        let routes = HashMap::from([(s.atlanta.name, HashMap::from([]))]);
        let path = shortest_path(&routes, s.atlanta.name, s.atlanta.name);
        assert_eq!(path, vec![s.atlanta.name]);
    }
}
