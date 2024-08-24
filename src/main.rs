mod constants;
mod map;
mod block;
mod field;
mod graphic_controller;
mod block_controller;
mod game_controller;

use game_controller::GameController;
use macroquad::prelude::*;
use constants::window_constants::{
    WINDOW_TITLE,
    WINDOW_WIDTH,
    WINDOW_HEIGHT,
};

fn window_conf() -> Conf {
    Conf {
        window_title: WINDOW_TITLE.to_string(),
        window_width: WINDOW_WIDTH as i32,
        window_height: WINDOW_HEIGHT as i32,
        ..Default::default() // Use default values for other settings
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut gc: GameController = GameController::new();
    gc.init_game();
    loop {
        gc.tick().await;
        if is_key_down(KeyCode::D) {
            gc.do_move(KeyCode::D)
        }
        if is_key_down(KeyCode::A) {
            gc.do_move(KeyCode::A)
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