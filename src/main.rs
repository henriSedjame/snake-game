mod draw;
mod snake;
mod direction;
mod game;

extern crate rand;
extern crate piston_window;

use piston_window::*;
use piston_window::types::Color;
use game::Game;
use draw::to_coord_u32;
use std::option::Option::Some;

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
    let (width, height) = (30, 30);

    let mut game_board: PistonWindow = WindowSettings::new(
        "Snake",
        [to_coord_u32(width), to_coord_u32(height)]
    )
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game::new(width, height);

    while let Some(event) = game_board.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }

        game_board.draw_2d(&event, |ctx, g, _|{
            clear(BACK_COLOR, g);
            game.draw(&ctx, g);
        });

        event.update(|arg|{
            game.update(arg.dt);
        });
        
    }

}
