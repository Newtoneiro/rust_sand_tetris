mod constants;
mod map;
mod field;
mod graphic_controller;

use macroquad::prelude::*;
use constants::window_constants::{
    WINDOW_TITLE,
    WINDOW_WIDTH,
    WINDOW_HEIGHT,
};
use constants::map_constants::{
    MAP_WIDTH,
    MAP_HEIGHT,
};
use map::Map;
use graphic_controller::GraphicController;

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
    let map = Map::new(MAP_WIDTH, MAP_HEIGHT);
    loop {
        GraphicController::draw_fields(map.get_fields_to_draw()).await;
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