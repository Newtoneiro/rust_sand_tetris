mod controllers;
mod objects;
mod constants;

use constants::window_constants::{WINDOW_HEIGHT, WINDOW_TITLE, WINDOW_WIDTH};
use controllers::game_controller::GameController;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: WINDOW_TITLE.to_string(),
        window_width: WINDOW_WIDTH as i32,
        window_height: WINDOW_HEIGHT as i32,
        window_resizable: false,
        ..Default::default() // Use default values for other settings
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut gc: GameController = GameController::new();
    gc.reset_game();
    loop {
        gc.tick().await;
        if is_key_down(KeyCode::D) {
            gc.do_move(KeyCode::D)
        }
        if is_key_down(KeyCode::A) {
            gc.do_move(KeyCode::A)
        }
        if is_key_down(KeyCode::S) {
            gc.do_move(KeyCode::S)
        }
        if is_key_pressed(KeyCode::E) {
            gc.do_move(KeyCode::E)
        }
        if is_key_pressed(KeyCode::Q) {
            gc.do_move(KeyCode::Q)
        }
        if is_key_pressed(KeyCode::R) {
            gc.reset_game()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn init() {
        assert!(true);
    }

    #[test]
    fn test_window_conf() {
        let conf = window_conf();

        assert_eq!(conf.window_title, WINDOW_TITLE);
        assert_eq!(conf.window_width, WINDOW_WIDTH as i32);
        assert_eq!(conf.window_height, WINDOW_HEIGHT as i32);
    }
}
