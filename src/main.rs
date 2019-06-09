extern crate piston;
extern crate piston_window;

use std::fs::File;
use std::io::prelude::*;
use piston_window::*;

const MAZE_SIZE: usize = 10;

// From IOStuff.c in c-maze
const TILE_SIZE: u32 = 16;

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

fn art_rect(img: Art) -> [f64; 4] {
    let ti = img as u32;
    let sheet_width: u32 = TILE_SIZE * 10; // 10 enum variants
    [
        (ti % (sheet_width / TILE_SIZE) * TILE_SIZE) as f64,
        (ti / (sheet_width / TILE_SIZE) * TILE_SIZE) as f64,
        TILE_SIZE as f64,
        TILE_SIZE as f64,
    ]
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
    println!("Width is {}", width);

    let ti = 1;

    let tile_src: [f64; 4] = [
        (ti % (width / TILE_SIZE) * TILE_SIZE) as f64,
        (ti / (width / TILE_SIZE) * TILE_SIZE) as f64,
        TILE_SIZE as f64,
        TILE_SIZE as f64,
    ];

    let img = Image::new();

    // while let Some(e) = window.next() {
    //     window.draw_2d(&e, |c,g,_| {
    //         clear([1.0; 4], g);

    //         img.src_rect(art_rect(Art::Wall)).draw(
    //             &tilesheet,
    //             &DrawState::default(),
    //             c.transform.zoom(5.0),
    //             g,
    //         );
    //         // image(&tilesheet, c.transform.zoom(2.0), g);
    //     });
    // };

    let mut x = 0.0;
    let mut y = 0.0;

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _| {
            clear([1.0; 4], g);
            for y in 0..MAZE_SIZE {
                for x in 0..MAZE_SIZE {
                    match maze.map[x][y] {
                        // Tile::Wall => rectangle([1.0, 0.0, 0.0, 1.0],
                        //                         [25.0 * x as f64, 25.0 * y as f64, 25.0, 25.0],
                        //                         c.transform, g),
                        Tile::Wall => img.src_rect(art_rect(Art::Wall)).draw(
                            &tilesheet,
                            &DrawState::default(),
                            c.transform.trans(
                                TILE_SIZE as f64 * x as f64,
                                TILE_SIZE as f64 * y as f64
                            ),
                            g
                        ),
                        Tile::Space => img.src_rect(art_rect(Art::Space)).draw(
                            &tilesheet,
                            &DrawState::default(),
                            c.transform.trans(
                                TILE_SIZE as f64 * x as f64,
                                TILE_SIZE as f64 * y as f64
                            ),
                            g
                        ),
                        _ => ()
                    }
                    if maze.start.x == x && maze.start.y == y {
                        img.src_rect(art_rect(Art::CDown)).draw(
                            &tilesheet,
                            &DrawState::default(),
                            c.transform.trans(
                                TILE_SIZE as f64 * x as f64,
                                TILE_SIZE as f64 * y as f64
                            ),
                            g
                        );
                    }
                    if maze.goal.x == x && maze.goal.y == y {
                        img.src_rect(art_rect(Art::Goal)).draw(
                            &tilesheet,
                            &DrawState::default(),
                            c.transform.trans(
                                TILE_SIZE as f64 * x as f64,
                                TILE_SIZE as f64 * y as f64
                            ),
                            g
                        );
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
