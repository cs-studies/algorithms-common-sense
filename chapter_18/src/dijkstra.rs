use std::collections::{HashMap, HashSet};

type Name = &'static str;
type Price = u16;
type Route = HashMap<Name, Price>;

pub struct City {
    pub name: Name,
}

impl City {
    pub fn new(name: Name) -> Self {
        Self { name }
    }
}

pub fn shortest_path(
    routes: &HashMap<Name, Route>,
    start: Name,
    destination: Name,
) -> Vec<Name> {
    let mut prices: HashMap<Name, Price> = HashMap::new();
    let mut stopovers: HashMap<Name, Name> = HashMap::new();
    let mut unvisited: HashSet<Name> = HashSet::new();
    let mut visited: HashSet<Name> = HashSet::new();

    prices.insert(start, 0);

    let mut current_opt = Some(start);

    while let Some(current) = current_opt {
        visited.insert(current);
        unvisited.remove(&current);

        if let Some(neighbors) = routes.get(&current) {
            let current_price = prices
                .get(&current)
                .copied()
                .expect("current city must have a known price");
            for (&adjacent, &price) in neighbors {
                if !visited.contains(&adjacent) {
                    unvisited.insert(adjacent);
                }
                let new_price = current_price.saturating_add(price);
                if prices.get(&adjacent).is_none_or(|&p| new_price < p) {
                    prices.insert(adjacent, new_price);
                    stopovers.insert(adjacent, current);
                }
            }
        }

        current_opt = unvisited
            .iter()
            .min_by_key(|name| prices.get(*name).copied().unwrap_or(Price::MAX))
            .copied();
    }

    let mut path = Vec::new();
    let mut current = destination;

    while current != start {
        path.push(current);
        current = stopovers
            .get(&current)
            .copied()
            .expect("stopovers must contain cities unless start is reached");
    }

    path.push(start);
    path.reverse();
    path
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
