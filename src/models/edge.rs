use std::error::Error;
use std::fmt::Display;

use super::coordinate::Coordinate;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Edge {
    coordinate: Coordinate,
    fixed: bool,
}

impl Edge {
    pub fn position(self) -> Coordinate {
        self.coordinate
    }

    pub fn move_to(&mut self, coord: Coordinate) -> Result<(), EdgeFixedError> {
        match self.fixed {
            true => Err(EdgeFixedError {
                source: self.coordinate,
                destination: coord,
            }),
            false => {
                self.coordinate = coord;
                Ok(())
            }
        }
    }

    pub fn fix(&mut self) {
        self.fixed = true;
    }

    pub fn unfix(&mut self) {
        self.fixed = false;
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

impl Display for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.coordinate)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EdgeFixedError {
    source: Coordinate,
    destination: Coordinate,
}

impl Display for EdgeFixedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "The edge at {} cannot be moved to {} as it is fixed in place",
            self.source, self.destination,
        )
    }
}

impl Error for EdgeFixedError {}
