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
            .map(|(vl, vr)| vl * vr)
            .reduce(|acc, v| acc + v)
            .unwrap()
    }
}

#[test]
fn test_add_coordinate() {
    let coord1 = Coordinate {data: [1., 2., 3.]};
    let coord2 = Coordinate {data: [1., 4., 9.]};

    let actual = coord1 + coord2;
    let expected = Coordinate {data: [2., 6., 12.]};
    assert_eq!(actual, expected);
    assert_ne!(actual, coord1);
}

#[test]
fn test_sub_coordinate() {
    let coord1 = Coordinate {data: [1., 2., 3.]};
    let coord2 = Coordinate {data: [1., 4., 9.]};

    let actual = coord1 - coord2;
    let expected = Coordinate {data: [0., -2., -6.]};
    assert_eq!(actual, expected);
    assert_ne!(actual, coord1);
}

#[test]
fn test_mul_coordinate() {
    let coord1 = Coordinate {data: [1., 2., 3.]};
    let coord2 = Coordinate {data: [1., 4., 9.]};

    let actual1 = coord1 * coord2;
    let expected1 = 36.;
    assert_eq!(actual1, expected1);

    let coord3 = Coordinate {data: [1., 1., -1.]};
    let actual2 = coord1 * coord3;
    let expected2 = 0.;
    assert_eq!(actual2, expected2);
}
