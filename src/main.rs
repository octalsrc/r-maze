extern crate piston;
extern crate piston_window;

use piston_window::*;

fn main() {
    let mut window: PistonWindow = 
        WindowSettings::new("Hello Piston!", [640, 480])
        .exit_on_esc(true).build().unwrap();

    let mut x = 0.0;
    let mut y = 0.0;

    while let Some(e) = window.next() {
        if let Some(Button::Keyboard(Key::D)) = e.press_args() {
            x += 100.0;
        }
        if let Some(Button::Keyboard(Key::A)) = e.press_args() {
            x -= 100.0;
        }
        if let Some(Button::Keyboard(Key::S)) = e.press_args() {
            y += 100.0;
        }
        if let Some(Button::Keyboard(Key::W)) = e.press_args() {
            y -= 100.0;
        }

        window.draw_2d(&e, |c, g, _device| {
            clear([1.0; 4], g);
            rectangle([1.0, 0.0, 0.0, 1.0], // red
                      [x, y, 100.0, 100.0],
                      c.transform, g);
        });
    }
}
