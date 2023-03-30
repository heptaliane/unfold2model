use std::fmt::Display;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coordinate {
    pub data: [f64; 3],
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.data.map(|v| v.to_string()).join(", "))
    }
}

impl IntoIterator for Coordinate {
    type Item = f64;
    type IntoIter = std::array::IntoIter<Self::Item, 3>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl Add for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: Self) -> Self::Output {
        Coordinate {
            data: [
                self.data[0] + rhs.data[0],
                self.data[1] + rhs.data[1],
                self.data[2] + rhs.data[2],
            ],
        }
    }
}

impl Sub for Coordinate {
    type Output = Coordinate;

    fn sub(self, rhs: Self) -> Self::Output {
        Coordinate {
            data: [
                self.data[0] - rhs.data[0],
                self.data[1] - rhs.data[1],
                self.data[2] - rhs.data[2],
            ],
        }
    }
}

impl Mul for Coordinate {
    type Output = f64;

    fn mul(self, rhs: Self) -> Self::Output {
        self.data
            .iter()
            .zip(rhs.data.iter())
            .map(|(v1, v2)| v1 * v2)
            .reduce(|acc, v| acc + v)
            .unwrap()
    }
}
