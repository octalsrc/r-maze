extern crate piston;
extern crate piston_window;

use std::fs::File;
use std::io::prelude::*;
use piston_window::*;

const MAZE_SIZE: usize = 10;

// From IOStuff.c in c-maze
const ART_SIZE: u32 = 16;

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

enum Art {
    Error,
    Space,
    Wall,
    Dark1,
    Dark2,
    Goal,
    CUp,
    CDown,
    CRight,
    CLeft,
}

impl Art {
    fn image(self) -> Image {
        let tile_index = self as u32;
        let sheet_width: u32 = ART_SIZE * 10;
        let rect = [
            (tile_index % (sheet_width / ART_SIZE) * ART_SIZE) as f64,
            (tile_index / (sheet_width / ART_SIZE) * ART_SIZE) as f64,
            ART_SIZE as f64,
            ART_SIZE as f64,
        ];
        Image::new().src_rect(rect)
    }
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
    if width != ART_SIZE * 10 {
        panic!("Wrong tilesheet size.");
    }

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _| {
            clear([1.0; 4], g);
            for y in 0..MAZE_SIZE {
                for x in 0..MAZE_SIZE {

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

                    match maze.map[x][y] {
                        Tile::Wall => draw_tile(Art::Wall),
                        Tile::Space => draw_tile(Art::Space),
                        _ => ()
                    }
                    if (maze.goal.x, maze.goal.y) == (x,y) {
                        draw_tile(Art::Goal);
                    }
                    if (maze.start.x, maze.start.y) == (x,y) {
                        draw_tile(Art::CDown);
                    }

                }
            }
        });
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
