type Int = isize;

pub const DIR_RESOLUTION: Int = 8;

/// Direction in maze space
#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub struct Dir {
    i: Int,
}

fn dir(i: Int) -> Dir {
    let i2 = i % DIR_RESOLUTION;
    let i3 = if i2 < 0 {
        i2 + 8
    } else {
        i2
    };
    Dir{i: i3}
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
    pub fn adj(self, dir: Dir) -> Loc {
        let (x,y) = dir.offset();
        Loc{x: self.x + x, y: self.y + y}
    }
    /// Get location as (f64,f64) coordinates, for drawing graphics
    /// and so on
    pub fn as_coords(self) -> (f64,f64) {
        (self.x as f64, self.y as f64)
    }

    pub fn sub(self, l: Loc) -> Loc {
        Loc{x: self.x - l.x, y: self.y - l.y}
    }
    pub fn add(self, l: Loc) -> Loc {
        Loc{x: self.x + l.x, y: self.y + l.y}
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct FineLoc {
    pub base: Loc,
    offsets: (f64,f64),
}

impl FineLoc {
    pub fn new(base: Loc, offsets: (f64,f64)) -> FineLoc {
        let x = base.x;
        let y = base.y;
        let xo = offsets.0;
        let yo = offsets.1;
        FineLoc{
            base: Loc{ x: x + (xo as isize), y: y + (yo as isize) },
            offsets: (xo.fract(), yo.fract()),
        }
    }

    /// Create FineLoc centered on a basic Loc.
    pub fn from_loc(base: Loc) -> FineLoc {
        FineLoc::new(base, (0.0,0.0))
    }

    pub fn from_coords(coords: (f64,f64)) -> FineLoc {
        FineLoc::new(Loc{x:0,y:0}, coords)
    }

    pub fn get_offsets(self) -> (f64,f64) {
        self.offsets
    }

    pub fn step(self, dir: Dir) -> FineLoc {
        FineLoc{
            base: self.base.adj(dir),
            offsets: self.offsets,
        }
    }

    pub fn add(self, l: FineLoc) -> FineLoc {
        let offsets = (self.offsets.0 + l.offsets.0,
                       self.offsets.1 + l.offsets.1);
        FineLoc::new(self.base.add(l.base), offsets)
    }

    pub fn sub(self, l: FineLoc) -> FineLoc {
        let offsets = (self.offsets.0 - l.offsets.0,
                       self.offsets.1 - l.offsets.1);
        FineLoc::new(self.base.sub(l.base), offsets)
    }

    pub fn as_coords(self) -> (f64,f64) {
        (self.base.x as f64 + self.offsets.0,
         self.base.y as f64 + self.offsets.1)
    }

    // /// Snap to base loc in direction of current offset, or stay in
    // /// place if no offset.
    // pub fn realign(self) -> FineLoc {
    //     match self.offset {
    //         Some((dir,_)) => FineLoc::from_loc(self.base_loc.adj(dir)),
    //         None => self,
    //     }
    // }

    // /// Flatten to basic f64 coordinates.
    // pub fn as_coords(self) -> (f64,f64) {
    //     let (x,y) = self.base_loc.as_coords();
    //     match self.offset {
    //         Some((dir,dist)) => {
    //             if dir == Dir::north() {
    //                 (x, y - dist)
    //             } else if dir == Dir::south() {
    //                 (x, y + dist)
    //             } else if dir == Dir::east() {
    //                 (x + dist, y)
    //             } else if dir == Dir::west() {
    //                 (x - dist, y)
    //             } else {
    //                 panic!("Weird Dir value: {:?}", dir);
    //             }
    //         },
    //         None => (x,y)
    //     }
    // }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum RouteResult {
    Complete(Loc),
    InProgress(TileRoute),
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct TileRoute {
    pub start: Loc,
    dir: Dir,
    progress: f64,
}

impl TileRoute {
    pub fn new(start: Loc, dir: Dir) -> TileRoute {
        TileRoute{start, dir, progress: 0.0}
    }
    pub fn dest(self) -> Loc {
        self.start.adj(self.dir)
    }
    pub fn advance(mut self, delta: f64) -> RouteResult {
        self.progress += delta;
        if self.progress >= 1.0 {
            RouteResult::Complete(self.dest())
        } else {
            RouteResult::InProgress(self)
        }
    }
    pub fn get_progress(self) -> f64 {
        self.progress
    }
    pub fn as_fineloc(self) -> FineLoc {
        let (xo,yo) = if self.dir == Dir::north() {
            (0.0, 0.0 - self.progress)
        } else if self.dir == Dir::south() {
            (0.0, self.progress)
        } else if self.dir == Dir::east() {
            (self.progress, 0.0)
        } else if self.dir == Dir::west() {
            (0.0 - self.progress, 0.0)
        } else {
            panic!("Weird Dir value: {:?}", self.dir)
        };
        FineLoc::new(self.start, (xo,yo))
    }
}
