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

#[test]
fn test_length() {
    let edge1 = Edge::from([0., 0., 0.]);
    let edge2 = Edge::from([1., 2., 3.]);
    let node1 = Node {edges: [Cell::from(edge1), Cell::from(edge2)]};

    let expected1 = 14f64.sqrt();
    let actual1 = node1.length();
    assert!((expected1 - actual1).abs() <= f64::EPSILON);

    let edge3 = Edge::from([2., 4., 6.]);
    let node2 = Node {edges: [Cell::from(edge2), Cell::from(edge3)]};

    let expected2 = 14f64.sqrt();
    let actual2 = node2.length();
    assert!((expected2 - actual2).abs() <= f64::EPSILON);

}
