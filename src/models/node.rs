use std::cell::Cell;

use super::edge::Edge;

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    edges: [Cell<Edge>; 2],
}

impl Node {
    pub fn length(&self) -> f64 {
        self.edges[0]
            .get()
            .position()
            .into_iter()
            .zip(self.edges[1].get().position().into_iter())
            .map(|(x1, x2)| (x1 - x2).powi(2))
            .reduce(|acc, v| acc + v)
            .unwrap_or(0.0)
            .sqrt()
    }
}
