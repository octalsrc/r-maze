type Int = isize;

pub const DIR_RESOLUTION: Int = 8;

/// Direction in maze space
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct Dir {
    i: Int,
}

fn dir(i: Int) -> Dir {
    Dir{i: i % DIR_RESOLUTION}
}

impl Dir {
    pub fn north() -> Dir { dir(0) }
    pub fn ne() -> Dir    { dir(1) }
    pub fn east() -> Dir  { dir(2) }
    pub fn se() -> Dir    { dir(3) }
    pub fn south() -> Dir { dir(4) }
    pub fn sw() -> Dir    { dir(5) }
    pub fn west() -> Dir  { dir(6) }
    pub fn nw() -> Dir    { dir(7) }
    fn offset(&self) -> (Int, Int) {
        match self.i {
            0 => (0,-1),
            1 => (1,-1),
            2 => (1,0),
            3 => (1,1),
            4 => (0,1),
            5 => (-1,1),
            6 => (-1,0),
            7 => (-1,-1),
            n => panic!(
                "Direction {} is out of range [0,{}].",
                n,
                DIR_RESOLUTION - 1,
            ),
        }
    }
    pub fn turn(&self, a: &Angle) -> Dir {
        dir(self.i + a.i)
    }
    pub fn as_int(&self) -> Int {
        self.i
    }
}

/// Relative direction
pub struct Angle {
    i: Int,
}

impl Angle {
    pub fn a45() -> Angle { Angle { i: DIR_RESOLUTION / 8 } }
    pub fn a90() -> Angle { Angle { i: DIR_RESOLUTION / 4 } }
    pub fn a180() -> Angle { Angle { i: DIR_RESOLUTION / 1 } }
    pub fn a360() -> Angle { Angle { i: DIR_RESOLUTION } }
    pub fn reverse(&self) -> Angle {
        Angle { i: &self.i * -1 }
    }
    pub fn as_dir(&self) -> Dir {
        Dir::north().turn(self)
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

    pub fn trans(&self, l: &Loc) -> Loc {
        Loc{x: self.x + l.x, y: self.y + l.y}
    }

    pub fn diff(&self, l: &Loc) -> Loc {
        Loc{x: self.x - l.x, y: self.y - l.y}
    }
    pub fn add(&self, l: &Loc) -> Loc {
        Loc{x: self.x + l.x, y: self.y + l.y}
    }
}
