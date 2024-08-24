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
    let mut map = Map::new(MAP_WIDTH, MAP_HEIGHT);
    let mut i: u32 = 0;
    // map.change_field_color(1, 9, RED); // Grain under
    // map.change_field_color(1, 8, RED); // Grain above
    
    // map.change_field_color(3, 9, RED); // left Grain above
    // map.change_field_color(3, 8, RED); // left Grain above
    loop {
        GraphicController::draw_fields(map.get_fields_to_draw()).await;
        map.tick();
        if i % 20 == 0 {
            map.change_field_color(50, 0, RED);
            // map.change_field_color(50, 1, RED);
            // map.change_field_color(51, 0, RED);
            // map.change_field_color(51, 1, RED);
        }
        i += 1;
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