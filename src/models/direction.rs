use super::coordinate::Coordinate;
use super::node::Node;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Direction {
    coordinate: Coordinate,
}

impl Direction {
    pub fn rotate_matrix(&self, theta: f64) -> [[f64; 3]; 3] {
        let [n1, n2, n3] = self.coordinate.data;
        let (cos_t, sin_t) = (theta.cos(), theta.sin());

        // Generate Rodrigues' rotation formula
        [
            [
                n1 * n1 * (1. - cos_t) + cos_t,
                n1 * n2 * (1. - cos_t) - n3 * sin_t,
                n1 * n3 * (1. - cos_t) + n2 * sin_t,
            ],
            [
                n1 * n2 * (1. - cos_t) + n3 * sin_t,
                n2 * n2 * (1. - cos_t) + cos_t,
                n2 * n3 * (1. - cos_t) - n1 * sin_t,
            ],
            [
                n1 * n3 * (1. - cos_t) - n2 * sin_t,
                n2 * n3 * (1. - cos_t) + n1 * sin_t,
                n3 * n3 * (1. - cos_t) + cos_t,
            ],
        ]
    }
}

impl From<Coordinate> for Direction {
    fn from(coord: Coordinate) -> Self {
        let length = (coord * coord).sqrt();

        Self {
            coordinate: Coordinate {
                data: [
                    coord.data[0] / length,
                    coord.data[1] / length,
                    coord.data[2] / length,
                ],
            },
        }
    }
}

impl From<Node> for Direction {
    fn from(node: Node) -> Self {
        Self::from(node.edges[0].get().position() - node.edges[1].get().position())
    }
}

impl From<[f64; 3]> for Direction {
    fn from(arr: [f64; 3]) -> Self {
        Self::from(Coordinate { data: arr })
    }
}
