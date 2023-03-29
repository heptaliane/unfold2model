use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coordinate {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl From<[f64; 3]> for Coordinate {
    fn from(arr: [f64; 3]) -> Self {
        Coordinate {
            x: arr[0],
            y: arr[1],
            z: arr[2],
        }
    }
}

impl Into<[f64; 3]> for Coordinate {
    fn into(self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }
}

impl IntoIterator for Coordinate {
    type Item = f64;
    type IntoIter = std::array::IntoIter<Self::Item, 3>;

    fn into_iter(self) -> Self::IntoIter {
        Into::<[Self::Item; 3]>::into(self).into_iter()
    }
}
