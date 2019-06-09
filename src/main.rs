extern crate piston;
extern crate piston_window;

use std::fs::File;
use std::io::prelude::*;
use piston_window::*;

const MAZE_SIZE: usize = 10;

#[derive(Copy, Clone, Debug)]
struct Loc {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone, Debug)]
enum Tile {
    Space,
    Wall,
}

#[derive(Copy, Clone, Debug)]
struct Maze {
    start: Loc,
    goal: Loc,
    map: [[Tile; MAZE_SIZE]; MAZE_SIZE],
}

fn parse_maze(fname: &str) -> std::io::Result<Maze> {
    let mut file = File::open(fname)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut start = Loc{x:0,y:0};
    let mut goal = Loc{x:0,y:0};
    let mut map = [[Tile::Space; MAZE_SIZE]; MAZE_SIZE];
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut brk: bool = false;
    for c in contents.chars() {
        match c {
            '.' => (),
            ' ' => (),
            '=' => map[x][y] = Tile::Wall,
            's' => start = Loc{x,y},
            'g' => goal = Loc{x,y},
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

fn main() {
    let maze: Maze = parse_maze("test-maze.txt").unwrap();

    let mut window: PistonWindow = 
        WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true).build().unwrap();

    let mut x = 0.0;
    let mut y = 0.0;

    while let Some(e) = window.next() {
        for y in 0..MAZE_SIZE {
            for x in 0..MAZE_SIZE {
            }
        }
    }

    // while let Some(e) = window.next() {
    //     if let Some(Button::Keyboard(Key::D)) = e.press_args() {
    //         x += 100.0;
    //     }
    //     if let Some(Button::Keyboard(Key::A)) = e.press_args() {
    //         x -= 100.0;
    //     }
    //     if let Some(Button::Keyboard(Key::S)) = e.press_args() {
    //         y += 100.0;
    //     }
    //     if let Some(Button::Keyboard(Key::W)) = e.press_args() {
    //         y -= 100.0;
    //     }

    //     window.draw_2d(&e, |c, g, _device| {
    //         clear([1.0; 4], g);
    //         rectangle([1.0, 0.0, 0.0, 1.0], // red
    //                   [x, y, 100.0, 100.0],
    //                   c.transform, g);
    //     });
    // }
}
