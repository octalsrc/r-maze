extern crate libc;
extern crate piston;
extern crate piston_window;

pub mod geometry;
pub mod mazes;
pub mod light;

use piston_window::*;
use std::collections::HashMap;

use crate::geometry::*;
use RouteResult::{Complete,InProgress};
use crate::mazes::*;
use crate::light::*;

/// Pixel width (and height) of artsheet tiles
const ART_SIZE: u32 = 16; // From IOStuff.c in c-maze

/// Number of tiles in the art sheet
const ART_NUM: u32 = 10;

/// Draw distance
const DRAW_DIST: isize = 10;

/// Distance cam falls behind before following
const CAM_DIST: f64 = 1.0;


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

fn offset_calc(loc: (f64,f64), dir: Dir, offset: f64) -> (f64,f64) {
    let (x,y) = loc;
    if dir == Dir::north() {
        (x, y - offset)
    } else if dir == Dir::south() {
        (x, y + offset)
    } else if dir == Dir::east() {
        (x + offset, y)
    } else if dir == Dir::west() {
        (x - offset, y)
    } else {
        (x,y)
    }
}

type LocMode = RouteResult;

struct Game {
    maze: Maze,
    loc: LocMode,
    intended_dir: Option<Dir>,
    speed: f64, // in tiles/sec
    dir: Dir,
    camera: FineLoc,
    battery: f64,
}

impl Game {
    /// Make a new game for a maze
    fn new(maze: Maze) -> Game {
        let start_loc = maze.start;
        Game{
            maze,
            loc: Complete(start_loc),
            dir: Dir::south(),
            speed: 5.0,
            intended_dir: None,
            camera: FineLoc::from_loc(start_loc),
            battery: 100.0,
        }
    }
    fn c_coords(&self) -> (f64,f64) {
        match self.loc {
            Complete(l) => l.as_coords(),
            InProgress(r) => r.as_fineloc().as_coords(),
        }
    }
    fn intend(&mut self, dir: Dir) {
        self.intended_dir = Some(dir);
    }
    fn unintend(&mut self, dir: Dir) {
        if let Some(d) = self.intended_dir {
            if d == dir {
                self.intended_dir = None;
            }
        }
    }
    /// Get tile at loc
    fn tile_at(&self, loc: Loc) -> Option<Tile> {
        match self.maze.map.get(&loc) {
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
        match self.loc {
            Complete(l) => self.tile_at(l.adj(dir)),
            InProgress(r) => self.tile_at(r.start.adj(dir)),
        }
    }
    /// Update position if in motion, otherwise set into motion if
    /// there is intent.
    fn update(&mut self, dt: f64) {
        match self.loc {
            InProgress(route) => {
                self.loc = route.advance(dt);
            },
            Complete(loc) => match self.intended_dir {
                Some(d) => {
                    self.dir = d;
                    if self.adj(d) == Some(Tile::Floor) {
                        self.loc = InProgress(TileRoute::new(loc,d));
                    }
                },
                None => (),
            },
        }
    }
    fn base_loc(&self) -> Loc {
        match self.loc {
            Complete(l) => l,
            InProgress(r) => r.as_fineloc().base,
        }
    }
    fn fine_loc(&self) -> FineLoc {
        match self.loc {
            Complete(l) => FineLoc::from_loc(l),
            InProgress(r) => r.as_fineloc(),
        }
    }
    // fn update(&mut self, dt: f64) {
    //     match self.loc.offset {
    //         // A Some value means we are in motion
    //         Some((d,o)) => {
    //             // Move according to speed
    //             let o2 = o + dt * self.speed;
    //             // Check if we have arrived at next tile
    //             if o2 >= 1.0 {
    //                 // if so, realign to next tile
    //                 self.loc = self.loc.realign();
    //             } else {
    //                 // else, advance our offset
    //                 self.loc.offset = Some((d,o2));
    //             }
    //         },
    //         // A None value means we are at rest, ready to move anew
    //         None => match self.intended_dir {
    //             Some(d) => {
    //                 self.dir = d;
    //                 if self.adj(d) == Some(Tile::Floor) {
    //                     self.loc.offset = Some((d,0.0));
    //                 }
    //             },
    //             None => (),
    //         },
    //     }
    // }
    fn settle_cam(&mut self) {
        let d = self.fine_loc().sub(self.camera).as_coords();
        if d.0 > CAM_DIST {
            self.camera = self.fine_loc().sub(FineLoc::from_coords((CAM_DIST,0.0)))
        } else if d.0 < -CAM_DIST {
            self.camera = self.fine_loc().sub(FineLoc::from_coords((-CAM_DIST,0.0)))
        } else if d.1 > CAM_DIST {
            self.camera = self.fine_loc().sub(FineLoc::from_coords((0.0,CAM_DIST)))
        } else if d.1 < CAM_DIST {
            self.camera = self.fine_loc().sub(FineLoc::from_coords((0.0,-CAM_DIST)))
        }
    }
    // fn settle_cam(&mut self) {
    //     let d = self.loc.base_loc.diff(self.camera);
    //     if d.x > CAM_DIST {
    //         self.camera.x = self.loc.base_loc.x - CAM_DIST
    //     } else if d.x < -CAM_DIST {
    //         self.camera.x = self.loc.base_loc.x + CAM_DIST
    //     }
    //     if d.y > CAM_DIST {
    //         self.camera.y = self.loc.base_loc.y - CAM_DIST
    //     } else if d.y < -CAM_DIST {
    //         self.camera.y = self.loc.base_loc.y + CAM_DIST
    //     }
    // }
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
        if let Some(args) = e.update_args() {
            game.battery -= args.dt * 4.0;
            game.update(args.dt);
        }
        if game.battery <= 5.0 {
            println!("Lose.");
            break;
        }
        match e.press_args() {
            Some(Button::Keyboard(k)) => match k {
                Key::W => game.intend(Dir::north()),
                Key::S => game.intend(Dir::south()),
                Key::A => game.intend(Dir::west()),
                Key::D => game.intend(Dir::east()),
                _ => (),
            }
            _ => (),
        }
        match e.release_args() {
            Some(Button::Keyboard(k)) => match k {
                Key::W => game.unintend(Dir::north()),
                Key::S => game.unintend(Dir::south()),
                Key::A => game.unintend(Dir::west()),
                Key::D => game.unintend(Dir::east()),
                _ => (),
            }
            _ => (),
        }

        game.settle_cam();
        let mut lums = HashMap::new();
        illuminate(&game.maze, &Source::mk_source(game.base_loc(), game.dir, game.battery), &mut lums);

        window.draw_2d(&e, |c, g, _| {
            clear([0.0; 4], g);

            let mut draw_tile_c = |cs: (f64,f64), a: Art| {
                let t = c.transform.trans(
                    ART_SIZE as f64 * cs.0,
                    ART_SIZE as f64 * cs.1,
                );
                a.image().draw(&tilesheet, &DrawState::default(), t, g);
            };

            let mut draw_tile = |l: FineLoc, a: Art| {
                draw_tile_c(l.as_coords(), a);
            };

            // Draw map tiles
            let draw_cam = FineLoc::from_loc(Loc{ x: DRAW_DIST, y: DRAW_DIST });
            let map_cam = game.camera.clone();

            for x in 0..(DRAW_DIST * 2 + 1) {
                for y in 0..(DRAW_DIST * 2 + 1) {
                    // The point on the screen we are filling in
                    let draw_loc = FineLoc::from_loc(Loc{x,y}).sub(FineLoc::from_coords(map_cam.get_offsets()));
                    // The location in the map we are representing
                    let map_loc = map_cam.sub(draw_cam.sub(draw_loc));
                    match (game.maze.map.get(&map_loc.base), lums.get(&map_loc.base)) {
                        (Some(Tile::Floor), Some(n)) => {
                            if !(*n < DARK2_LIGHT) {
                                draw_tile(draw_loc, Art::Floor);
                                if game.maze.goal == map_loc.base {
                                    draw_tile(draw_loc, Art::Goal);
                                }
                                if *n < DARK1_LIGHT {
                                    draw_tile(draw_loc, Art::Dark1);
                                }
                            }
                        },
                        (None, Some(n)) => {
                            if !(*n < DARK2_LIGHT) {
                                draw_tile(draw_loc, Art::Wall);
                                if *n < DARK1_LIGHT {
                                    draw_tile(draw_loc, Art::Dark1);
                                }
                            }
                        },
                        _ => draw_tile(draw_loc, Art::Dark2),
                    }
                }
            }

            // Draw character
            let d_loc = draw_cam.sub(map_cam.sub(game.fine_loc()));
            // let d_coords = (FineLoc{base: d_loc, offset: game.loc.offset}).as_coords();
            let d_coords = d_loc.as_coords();
            draw_tile_c(d_coords, game.c_art());

            rectangle([1.0,1.0,1.0,0.5], [1.0,1.0,50.0,16.0], c.transform, g);
            rectangle([0.0,0.0,0.0,1.0], [3.0,3.0,46.0,13.0], c.transform, g);
            rectangle([1.0,1.0,1.0,0.5], [5.0,5.0,42.0 * ((game.battery - 5.0) / 95.0),8.0], c.transform, g);

        });
        if game.base_loc() == game.maze.goal {
            println!("Win.");
            break;
        }
    }
}
