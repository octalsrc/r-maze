use libc::{c_int, c_float};

use std::collections::HashMap;

use crate::geometry::*;
use super::{Maze,Tile};


#[repr(C)]
pub struct DR_position {
    pub x: c_int,
    pub y: c_int,
}

#[repr(C)]
pub struct CTile {
    pub t: c_int,
    pub light: c_float,
    pub p: DR_position,
}

#[repr(C)]
pub struct CMaze {
    pub tiles: *const CTile,
    pub start_position: DR_position,
    pub goal_position: DR_position,
    pub size: c_int,
}

extern {
    fn generate_maze(
        size: c_int,
        twisty: c_int,
        swirly: c_int,
        branchy: c_int
    ) -> *mut CMaze;

    fn destroy_maze(maze: *mut CMaze);
}

unsafe fn translate_cmaze(cmaze: *const CMaze) -> Maze {
    // Read start and goal locations
    let start = Loc{
        x: (*cmaze).start_position.x as isize,
        y: (*cmaze).start_position.y as isize,
    };
    let goal = Loc{
        x: (*cmaze).goal_position.x as isize,
        y: (*cmaze).goal_position.y as isize,
    };

    // Collect locations of floor tiles
    let mut map = HashMap::new();
    let size = (*cmaze).size;
    for x in 0..size {
        for y in 0..size {
            // c-maze saves tiles in a 1D array. The maze size is used
            // to index it as a 2D grid of tiles.
            //
            // See get_tiletype in c_src/internals.c
            let tile_ptr = (*cmaze).tiles.offset((x * size + y) as isize);
            match (*tile_ptr).t {
                1 => { map.insert(Loc{x: x as isize,y: y as isize}, Tile::Floor); },
                _ => (),
            }
        }
    }

    Maze{start, goal, map}
}

/// Generate a random square-shaped maze.  The provided size will be
/// the length of one side of the maze.
pub fn generate(size: i32) -> Maze {
    unsafe {
        // Parameters come from their defaults in c-maze, which were
        // found with a bit of trial-and-error to make decent mazes.
        let cmaze = generate_maze(size, 70, 50, 30);
        let maze = translate_cmaze(cmaze);
        destroy_maze(cmaze); // free up tile array
        maze
    }
}
