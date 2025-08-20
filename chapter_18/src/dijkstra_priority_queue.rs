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
    use crate::dijkstra::City;

    #[test]
    fn test_shortest_path() {
        let atlanta = City::new("Atlanta");
        let boston = City::new("Boston");
        let chicago = City::new("Chicago");
        let denver = City::new("Denver");
        let elpaso = City::new("El Paso");

        let routes = HashMap::from([
            (
                atlanta.name,
                HashMap::from([(boston.name, 100), (denver.name, 160)]),
            ),
            (
                boston.name,
                HashMap::from([(chicago.name, 120), (denver.name, 180)]),
            ),
            (chicago.name, HashMap::from([(elpaso.name, 80)])),
            (
                denver.name,
                HashMap::from([(chicago.name, 40), (elpaso.name, 140)]),
            ),
            (elpaso.name, HashMap::from([])),
        ]);

        let path = shortest_path(&routes, atlanta.name, elpaso.name);
        assert_eq!(
            path,
            vec![atlanta.name, denver.name, chicago.name, elpaso.name]
        );
    }

    #[test]
    fn test_shortest_path_same_city() {
        let atlanta = City::new("Atlanta");
        let routes = HashMap::from([(atlanta.name, HashMap::from([]))]);
        let path = shortest_path(&routes, atlanta.name, atlanta.name);
        assert_eq!(path, vec![atlanta.name]);
    }
}
