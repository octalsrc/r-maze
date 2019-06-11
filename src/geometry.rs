type Int = isize;

/// Direction in maze space
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Dir {
    North,
    NE,
    East,
    SE,
    South,
    SW,
    West,
}

use Dir::*;

impl Dir {
    fn offset(&self) -> (Int, Int) {
        match self {
            North => (0,-1),
            NE => (1,-1),
            East => (1,0),
            SE => (1,1),
            South => (0,1),
            SW => (-1,1),
            West => (-1,0),
        }
    }
}

/// An address in maze space
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Loc {
    pub x: Int,
    pub y: Int,
}

impl Loc {
    /// Get adjacent location in given direction
    pub fn adj(&self, dir: Dir) -> Loc {
        let (x,y) = dir.offset();
        Loc{x: self.x + x, y: self.y + y}
    }
    /// Get location as (f64,f64) coordinates, for drawing graphics
    /// and so on
    pub fn as_coords(&self) -> (f64,f64) {
        (self.x as f64, self.y as f64)
    }
}
