extern crate piston;
extern crate piston_window;

use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use piston_window::*;

/// Static maze width (and height)
const MAZE_SIZE: isize = 10;

/// Pixel width (and height) of artsheet tiles
const ART_SIZE: u32 = 16; // From IOStuff.c in c-maze

/// An address in the maze
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Loc {
    x: isize,
    y: isize,
}

impl Loc {
    fn adj(&self, dir: Dir) -> Loc {
        Loc{
            x: self.x + dir.xoff(),
            y: self.y + dir.yoff()
        }
    }
}

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
    let mut loc: Loc = Loc{x:0,y:0};
    let mut brk: bool = false;
    for c in contents.chars() {
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
            loc.x = 0;
            loc.y += 1;
        } else {
            loc.x += 1;
        }
    }
    Ok(Maze{start, goal, map})
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum Dir {
    North,
    South,
    East,
    West,
}

impl Dir {
    fn xoff(&self) -> isize {
        match self {
            Dir::East => 1,
            Dir::West => -1,
            _ => 0
        }
    }
    fn yoff(&self) -> isize {
        match self {
            Dir::North => -1,
            Dir::South => 1,
            _ => 0
        }
    }
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
        Game{maze, loc, dir: Dir::South}
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
        match self.dir {
            Dir::North => Art::CNorth,
            Dir::South => Art::CSouth,
            Dir::East => Art::CEast,
            Dir::West => Art::CWest,
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
                Key::W => game.step(Dir::North),
                Key::S => game.step(Dir::South),
                Key::A => game.step(Dir::West),
                Key::D => game.step(Dir::East),
                _ => (),
            }
            _ => (),
        }

        window.draw_2d(&e, |c, g, _| {
            clear([0.0; 4], g);
            for y in 0..MAZE_SIZE {
                for x in 0..MAZE_SIZE {

                    let here = Loc{x,y};

                    let mut draw_tile = |a: Art| {
                        a.image().draw(
                            &tilesheet,
                            &DrawState::default(),
                            c.transform.trans(
                                ART_SIZE as f64 * x as f64,
                                ART_SIZE as f64 * y as f64
                            ),
                            g
                        );
                    };

                    match game.tile_at(&here) {
                        Some(Tile::Wall) => draw_tile(Art::Wall),
                        Some(Tile::Space) => draw_tile(Art::Space),
                        _ => draw_tile(Art::Error),
                    }
                    if game.maze.goal == here {
                        draw_tile(Art::Goal);
                    }
                    if game.loc == here {
                        draw_tile(game.c_art());
                    }

                }
            }
        });
    }
}
