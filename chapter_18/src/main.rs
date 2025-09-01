mod dijkstra;
mod dijkstra_priority_queue;
mod exercises;
mod graph;
mod weighted_graph;

use dijkstra::shortest_path;
use graph::{breadth_first, depth_first};
use weighted_graph::WeightedVertex;

fn main() {
    println!("\n*** Chapter 18 ***\n");

    let (alice, bob, cynthia) = graph::sample();

    dbg!(&alice);
    dbg!(&bob);
    dbg!(&cynthia);

    println!("\nTraverse depth-first: ");

    depth_first::traverse(&alice);

    println!("\nSearch depth-first:");
    println!(
        "Found Bob: {}",
        depth_first::search(&alice, &bob.borrow().value)
    );
    println!("Found Diana: {}", depth_first::search(&alice, &"Diana"));

    println!("\nTraverse breadth-first: ");
    breadth_first::traverse(&alice);

    let mut dallas = WeightedVertex::new("Dallas");
    let mut toronto = WeightedVertex::new("Toronto");

    dallas.add_neighbor(toronto.value, 138);
    toronto.add_neighbor(dallas.value, 216);

    println!("\n{:?}", dallas);
    println!("{:?}", toronto);

    println!("\nDijkstra's algorithm:");

    let s = dijkstra::sample::data();
    println!(
        "{:?}",
        shortest_path(&s.routes, s.atlanta.name, s.elpaso.name)
    );

    println!("\n Dijkstra's algorithm with priority queue:");

    println!(
        "{:?}",
        dijkstra_priority_queue::shortest_path(
            &s.routes,
            s.atlanta.name,
            s.elpaso.name
        )
    );

    //// Exercises
    println!("\n*** Exercises ***\n");

    println!("\nSearch breadth-first:");
    println!("Found Bob: {}", exercises::bfs(&alice, &bob.borrow().value));
    println!("Found Diana: {}", exercises::bfs(&alice, &"Diana"));

    println!("\nShortest path:");
    let s = exercises::sample();
    let path = exercises::shortest_path(&s.0, &s.2.borrow().value);
    println!("{:?}", path);
}
