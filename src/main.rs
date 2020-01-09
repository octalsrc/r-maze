extern crate libc;
extern crate piston;
extern crate piston_window;

pub mod geometry;
pub mod mazes;
pub mod light;

use piston_window::*;
use std::collections::HashMap;

use crate::geometry::*;
use crate::mazes::*;
use crate::light::*;

/// Pixel width (and height) of artsheet tiles
const ART_SIZE: u32 = 16; // From IOStuff.c in c-maze

/// Number of tiles in the art sheet
const ART_NUM: u32 = 10;

/// Draw distance
const DRAW_DIST: isize = 10;

/// Distance cam falls behind before following
const CAM_DIST: isize = 1;


/// Names for the tiles in the art sheet
enum Art {
    Error,
    Floor,
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

struct Game {
    maze: Maze,
    loc: Loc,
    dir: Dir,
    camera: Loc,
}

impl Game {
    /// Make a new game for a maze
    fn new(maze: Maze) -> Game {
        let loc = maze.start.clone();
        Game{
            maze,
            loc: loc.clone(),
            dir: Dir::south(),
            camera: loc.clone(),
        }
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
            if self.adj(dir) == Some(Tile::Floor) {
                self.loc = self.loc.adj(dir);
            }
        } else {
            self.dir = dir;
        }
    }
    fn settle_cam(&mut self) {
        let d = &self.loc.diff(&self.camera);
        if d.x > CAM_DIST {
            self.camera.x = self.loc.x - CAM_DIST
        } else if d.x < -CAM_DIST {
            self.camera.x = self.loc.x + CAM_DIST
        }
        if d.y > CAM_DIST {
            self.camera.y = self.loc.y - CAM_DIST
        } else if d.y < -CAM_DIST {
            self.camera.y = self.loc.y + CAM_DIST
        }
    }
}

fn main() {
    let mut game: Game = Game::new(maze_gen::generate(20));

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

        game.settle_cam();
        let mut lums = HashMap::new();
        illuminate(&game.maze, &Source::mk_source(&game.loc, &game.dir), &mut lums);

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
            let draw_cam = Loc{ x: DRAW_DIST, y: DRAW_DIST };
            let map_cam = game.camera.clone();

            for x in 0..(DRAW_DIST * 2 + 1) {
                for y in 0..(DRAW_DIST * 2 + 1) {
                    // The point on the screen we are filling in
                    let draw_loc = Loc{x,y};
                    // The location in the map we are representing
                    let map_loc = map_cam.diff(&draw_cam.diff(&draw_loc));
                    match (game.maze.map.get(&map_loc), lums.get(&map_loc)) {
                        (Some(Tile::Floor), Some(n)) => {
                            if !(*n < DARK2_LIGHT) {
                                draw_tile(&draw_loc, Art::Floor);
                                if game.maze.goal == map_loc {
                                    draw_tile(&draw_loc, Art::Goal);
                                }
                                if *n < DARK1_LIGHT {
                                    draw_tile(&draw_loc, Art::Dark1);
                                }
                            }
                        },
                        (None, Some(n)) => {
                            if !(*n < DARK2_LIGHT) {
                                draw_tile(&draw_loc, Art::Wall);
                                if *n < DARK1_LIGHT {
                                    draw_tile(&draw_loc, Art::Dark1);
                                }
                            }
                        },
                        _ => draw_tile(&draw_loc, Art::Dark2),
                    }
                    if game.loc == map_loc {
                        draw_tile(&draw_loc, game.c_art());
                    }
                }
            }

        });
        if game.loc == game.maze.goal {
            break;
        }
    }
}
