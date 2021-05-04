/// A coordinate on a hex grid, representing distances along the various directions of travel
/// Invariant: In order to represent a valid hex coordinate, q + r + s must equal 0
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct HexCoord {
    pub q: isize,
    pub r: isize,
    pub s: isize,
}

// The directions you can move on a hex grid
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Direction {
    None,
    North,
    South,
    Northeast,
    Southwest,
    Northwest,
    Southeast,
}
impl Direction {
    pub fn opposite(&self) -> Self {
        use Direction::*;
        match self {
            None => None,
            North => South,
            South => North,
            Northeast => Southwest,
            Southwest => Northeast,
            Northwest => Southeast,
            Southeast => Northwest,
        }
    }
}

/// All directions, for convenient enumeration
pub const DIRECTIONS: &[Direction] = &[
    Direction::North,
    Direction::Northeast,
    Direction::Southeast,
    Direction::South,
    Direction::Southwest,
    Direction::Northwest,
];

impl HexCoord {
    /// The origin of an infinite hex grid
    pub fn origin() -> Self {
        HexCoord { q: 0, r: 0, s: 0 }
    }
    /// Construct a hex coordinate from two pieces of information, enforcing the invariant on the third
    pub fn new(q: isize, r: isize) -> Self {
        HexCoord { q, r, s: -q - r }
    }

    /// The coordinate to the north
    pub fn north(&self) -> Self {
        Self::new(self.q + 0, self.r - 1)
    }
    /// The coordinate to the south
    pub fn south(&self) -> Self {
        Self::new(self.q + 0, self.r + 1)
    }

    /// The coordinate to the northeast
    pub fn northeast(&self) -> Self {
        Self::new(self.q + 1, self.r - 1)
    }
    /// The coordinate to the southwest
    pub fn southwest(&self) -> Self {
        Self::new(self.q - 1, self.r + 1)
    }

    /// The coordinate to the northwest
    pub fn northwest(&self) -> Self {
        Self::new(self.q - 1, self.r + 0)
    }
    /// The coordinate to the southeast
    pub fn southeast(&self) -> Self {
        Self::new(self.q + 1, self.r + 0)
    }

    /// The coordinate in a specific direction
    pub fn neighbor(&self, dir: Direction) -> Self {
        use Direction::*;
        match dir {
            None => self.clone(),
            North => self.north(),
            South => self.south(),
            Northeast => self.northeast(),
            Southwest => self.southwest(),
            Northwest => self.northwest(),
            Southeast => self.southeast(),
        }
    }

    /// Yield the neighbor coordinates, starting from North and going clockwise
    pub fn neighbors<'a>(&'a self) -> impl Iterator<Item = HexCoord> + 'a {
        struct NeighborIter<'a> {
            c: &'a HexCoord,
            iter: std::slice::Iter<'a, Direction>,
        }
        impl<'a> Iterator for NeighborIter<'a> {
            type Item = HexCoord;
            fn next(&mut self) -> Option<Self::Item> {
                self.iter.next().map(|d| self.c.neighbor(d.clone()))
            }
        }
        NeighborIter {
            c: self,
            iter: DIRECTIONS.iter(),
        }
    }
}
