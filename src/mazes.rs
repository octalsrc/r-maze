use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

use crate::geometry::*;

pub struct MazeGen {
    pub size: usize,
    pub twisty: f64,
    pub swirly: f64,
    pub branchy: f64,
}

impl MazeGen {
    pub fn generate(&self) -> Maze {
        unimplemented!()
    }
}

/// A descriptor of the features of a maze location
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Tile {
    Floor,
}

/// A map of maze tiles, with start and goal positions.  A correctly
/// constructed maze must have the start and goal positions on
/// accessible spaces.
#[derive(Clone, Debug)]
pub struct Maze {
    pub start: Loc,
    pub goal: Loc,
    pub map: HashMap<Loc,Tile>,
}

/// Parse a maze from a text file, in which '.' is a space, '=' is a
/// wall, 's' is the starting point and 'g' is the goal point.
///
/// There can only be one starting point and one goal.  If multiple
/// 's' or 'g' chars appear in the text file, the last occurrence of
/// each is used.
pub fn parse_maze(fname: &str) -> std::io::Result<Maze> {
    let mut file = File::open(fname)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut start = Loc{x:0,y:0};
    let mut goal = Loc{x:0,y:0};
    let mut map = HashMap::new();
    let mut brk: bool = false;
    let mut x: isize = 0;
    let mut y: isize = 0;
    for c in contents.chars() {
        let loc: Loc = Loc{x,y};
        match c {
            '.' => {map.insert(loc.clone(), Tile::Floor);},
            ' ' => {map.insert(loc.clone(), Tile::Floor);},
            '=' => (),
            's' => {
                map.insert(loc.clone(), Tile::Floor);
                start = loc.clone();
            },
            'g' => {
                map.insert(loc.clone(), Tile::Floor);
                goal = loc.clone();
            },
            '\n' => brk = true,
            _ => panic!("Don't know that char."),
        }
        if brk {
            brk = false;
            x = 0;
            y += 1;
        } else {
            x += 1;
        }
    }
    Ok(Maze{start, goal, map})
}
