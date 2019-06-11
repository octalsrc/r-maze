extern crate piston;
extern crate piston_window;

use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use piston_window::*;

pub mod geometry;

use crate::geometry::*;

/// Pixel width (and height) of artsheet tiles
const ART_SIZE: u32 = 16; // From IOStuff.c in c-maze


/// A descriptor of the features of a maze location
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum Tile {
    Space,
    Wall,
}

/// Number of tiles in the art sheet
const ART_NUM: u32 = 10;

/// Names for the tiles in the art sheet
enum Art {
    Error,
    Space,
    Wall,
    Dark1,
    Dark2,
    Goal,
    CNorth,
    CSouth,
    CEast,
    CWest,
}

impl Art {
    /// Create a ready-to-draw image from an art name
    fn image(self) -> Image {
        let tile_index = self as u32;
        let rect = [
            (tile_index % ART_NUM * ART_SIZE) as f64,
            (tile_index / ART_NUM * ART_SIZE) as f64,
            ART_SIZE as f64,
            ART_SIZE as f64,
        ];
        Image::new().src_rect(rect)
    }
}

/// A map of maze tiles, with start and goal positions.  A correctly
/// constructed maze must have the start and goal positions on
/// accessible spaces.
#[derive(Clone, Debug)]
struct Maze {
    start: Loc,
    goal: Loc,
    map: HashMap<Loc,Tile>,
}

/// Parse a maze from a text file, in which '.' is a space, '=' is a
/// wall, 's' is the starting point and 'g' is the goal point.
///
/// There can only be one starting point and one goal.  If multiple
/// 's' or 'g' chars appear in the text file, the last occurrence of
/// each is used.
fn parse_maze(fname: &str) -> std::io::Result<Maze> {
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
            '.' => {map.insert(loc.clone(), Tile::Space);},
            ' ' => {map.insert(loc.clone(), Tile::Space);},
            '=' => {map.insert(loc.clone(), Tile::Wall);},
            's' => {
                map.insert(loc.clone(), Tile::Space);
                start = loc.clone();
            },
            'g' => {
                map.insert(loc.clone(), Tile::Space);
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


struct Game {
    maze: Maze,
    loc: Loc,
    dir: Dir,
}

impl Game {
    /// Make a new game for a maze
    fn new(maze: Maze) -> Game {
        let loc = maze.start.clone();
        Game{maze, loc, dir: Dir::south()}
    }
    /// Get tile at loc
    fn tile_at(&self, loc: &Loc) -> Option<Tile> {
        match self.maze.map.get(loc) {
            Some(t) => Some(t.clone()),
            None => None,
        }
    }
    /// Get correct art for character's current state (just depends on
    /// which direction they're facing).
    fn c_art(&self) -> Art {
        match self.dir.as_int() {
            0 => Art::CNorth,
            2 => Art::CEast,
            4 => Art::CSouth,
            6 => Art::CWest,
            _ => Art::Error,
        }
    }
    /// Get tile adjecent to current loc, in given direction
    fn adj(&self, dir: Dir) -> Option<Tile> {
        self.tile_at(&self.loc.adj(dir))
    }
    /// Move in given direction if possible, or change direction
    fn step(&mut self, dir: Dir) {
        if dir == self.dir {
            if self.adj(dir) == Some(Tile::Space) {
                self.loc = self.loc.adj(dir);
            }
        } else {
            self.dir = dir;
        }
    }
}

fn main() {
    let mut game: Game = Game::new(parse_maze("test-maze.txt").unwrap());

    let mut window: PistonWindow = 
        WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true).build().unwrap();

    let mut texture_context = TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into(),
    };

    let tilesheet = Texture::from_path(
        &mut texture_context,
        "tilesheet.png",
        Flip::None,
        // Change filter from default "Linear" to "Nearest" in order
        // to not blur pixel-art tiles when scaling them up.
        &TextureSettings::new().mag(Filter::Nearest),
    ).unwrap();

    let (width, _): (u32,u32) = tilesheet.get_size();
    if width != ART_SIZE * ART_NUM {
        panic!("Wrong artsheet size.");
    }

    while let Some(e) = window.next() {
        match e.press_args() {
            Some(Button::Keyboard(k)) => match k {
                Key::W => game.step(Dir::north()),
                Key::S => game.step(Dir::south()),
                Key::A => game.step(Dir::west()),
                Key::D => game.step(Dir::east()),
                _ => (),
            }
            _ => (),
        }

        window.draw_2d(&e, |c, g, _| {
            clear([0.0; 4], g);

            let mut draw_tile = |l: &Loc, a: Art| {
                let (x,y) = l.as_coords();
                let t = c.transform.trans(
                    ART_SIZE as f64 * x,
                    ART_SIZE as f64 * y,
                );
                a.image().draw(&tilesheet, &DrawState::default(), t, g);
            };

            // Draw map tiles
            for (loc,tile) in game.maze.map.iter() {
                match tile {
                    Tile::Wall => draw_tile(loc, Art::Wall),
                    Tile::Space => draw_tile(loc, Art::Space),
                }
            }
            // Draw goal
            draw_tile(&game.maze.goal, Art::Goal);
            // Draw player character in correct orientation
            draw_tile(&game.loc, game.c_art());
        });
    }
}
