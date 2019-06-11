use std::collections::HashMap;

use crate::geometry::*;
use crate::mazes::*;

pub type Lum = f64;

// Vals come from c-maze
const DIVP: Lum = 2.0;
const DIVS: Lum = 3.0;

// Derived from c-maze full battery value (8000 * 4 / 1600)
pub const INIT_LIGHT: f64 = 20.0;

// In c-maze, light values of < 3 are dim
pub const DARK1_LIGHT: f64 = 3.0;

// In c-maze, light values of < 1 are total dark
pub const DARK2_LIGHT: f64 = 1.0;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum SourceKind {
    Primary,
    Left,
    Right,
}

#[derive(Copy, Clone, Debug)]
pub struct Source {
    power: Lum,
    dir: Dir,
    loc: Loc,
    kind: SourceKind,
}

impl Source {
    pub fn mk_source(loc: &Loc, dir: &Dir) -> Source {
        Source{power: INIT_LIGHT, dir: dir.clone(), loc: loc.clone(), kind: SourceKind::Primary}
    }
}

pub fn illuminate(maze: &Maze, source: &Source, map: &mut HashMap<Loc,Lum>) {
    map.insert(source.loc, source.power);
    if source.power >= 1.0 && maze.map.get(&source.loc) == Some(&Tile::Floor) {
        match source.kind {
            SourceKind::Primary => {
                let fsrc = Source{
                    power: source.power / DIVP,
                    dir: source.dir,
                    loc: source.loc.adj(source.dir),
                    kind: SourceKind::Primary,
                };
                illuminate(maze, &fsrc, map);
                let ldir = source.dir.turn(&Angle::a45().reverse());
                let lsrc = Source{
                    power: source.power / DIVS,
                    dir: ldir,
                    loc: source.loc.adj(ldir),
                    kind: SourceKind::Left,
                };
                illuminate(maze, &lsrc, map);
                let rdir = source.dir.turn(&Angle::a45());
                let rsrc = Source{
                    power: source.power / DIVS,
                    dir: rdir,
                    loc: source.loc.adj(rdir),
                    kind: SourceKind::Right,
                };
                illuminate(maze, &rsrc, map);
            }
            SourceKind::Left => {
                let dir = source.dir.turn(&Angle::a45().reverse());
                let src = Source {
                    power: source.power / DIVS,
                    dir,
                    loc: source.loc.adj(dir),
                    kind: SourceKind::Left,
                };
                illuminate(&maze, &src, map);
            },
            SourceKind::Right => {
                let dir = source.dir.turn(&Angle::a45());
                let src = Source {
                    power: source.power / DIVS,
                    dir,
                    loc: source.loc.adj(dir),
                    kind: SourceKind::Right,
                };
                illuminate(&maze, &src, map);
            },
        }
    }
}
