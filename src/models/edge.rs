use std::error::Error;
use std::fmt::Display;

use super::coordinate::Coordinate;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Edge {
    coordinate: Coordinate,
    pub fixed: bool,
}

impl Edge {
    pub fn position(self) -> Coordinate {
        self.coordinate
    }

    pub fn move_to(&mut self, coord: Coordinate) -> Result<(), EdgeFixedError> {
        match self.fixed {
            true => Err(EdgeFixedError {
                source: self.coordinate,
            }),
            false => {
                self.coordinate = coord;
                Ok(())
            }
        }
    }
}

impl From<Coordinate> for Edge {
    fn from(coord: Coordinate) -> Self {
        Edge {
            coordinate: coord,
            fixed: false,
        }
    }
}

impl From<[f64; 3]> for Edge {
    fn from(coord: [f64; 3]) -> Self {
        Edge::from(Coordinate { data: coord })
    }
}

impl Display for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.coordinate)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EdgeFixedError {
    source: Coordinate,
}

impl Display for EdgeFixedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "The edge at {} cannot be moved as it is fixed in place",
            self.source,
        )
    }
}

impl Error for EdgeFixedError {}
