mod dijkstra;
mod dijkstra_priority_queue;
mod graph;
mod weighted_graph;

use dijkstra::{shortest_path, City};
use graph::{breadth_first, depth_first, Vertex};
use std::collections::{HashMap, HashSet};
use weighted_graph::WeightedVertex;

fn main() {
    println!("\n*** Chapter 18 ***\n");

    let alice = Vertex::new("Alice");
    let bob = Vertex::new("Bob");
    let cynthia = Vertex::new("Cynthia");

    alice.borrow_mut().add_neighbor(&bob);
    alice.borrow_mut().add_neighbor(&cynthia);

    bob.borrow_mut().add_neighbor(&cynthia);
    cynthia.borrow_mut().add_neighbor(&bob);

    dbg!(&alice);
    dbg!(&bob);
    dbg!(&cynthia);

    println!("\nTraverse depth-first: ");

    depth_first::traverse(&alice.borrow(), &mut HashSet::new());

    println!("\nSearch depth-first:");
    println!(
        "Found Bob: {}",
        depth_first::search(&alice.borrow(), &"Bob", &mut HashSet::new())
    );
    println!(
        "Found Diana: {}",
        depth_first::search(&alice.borrow(), &"Diana", &mut HashSet::new())
    );

    println!("\nTraverse breadth-first: ");
    breadth_first::traverse(&alice);

    let mut dallas = WeightedVertex::new("Dallas");
    let mut toronto = WeightedVertex::new("Toronto");

    dallas.add_neighbor(toronto.value, 138);
    toronto.add_neighbor(dallas.value, 216);

    println!("\n{:?}", dallas);
    println!("{:?}", toronto);

    println!("\nDijkstra's algorithm:");

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

    println!("{:?}", shortest_path(&routes, atlanta.name, elpaso.name));

    println!("\n Dijkstra's algorithm with priority queue:");

    println!(
        "{:?}",
        dijkstra_priority_queue::shortest_path(
            &routes,
            atlanta.name,
            elpaso.name
        )
    );

    //// Exercises
    println!("\n*** Exercises ***\n");

    println!("\nSearch breadth-first:");
    println!("Found Bob: {}", breadth_first::search(&"Bob", &alice));

    println!("Found Diana: {}", breadth_first::search(&"Diana", &alice));
}
