use std::collections::HashMap;

#[derive(Debug)]
pub struct WeightedVertex<'a> {
    pub value: &'a str,
    neighbors: HashMap<&'a str, u32>,
}

impl<'a> WeightedVertex<'a> {
    pub fn new(value: &'a str) -> Self {
        Self {
            value,
            neighbors: HashMap::new(),
        }
    }

    pub fn add_neighbor(&mut self, neighbor: &'a str, weight: u32) {
        self.neighbors.insert(neighbor, weight);
    }
}
