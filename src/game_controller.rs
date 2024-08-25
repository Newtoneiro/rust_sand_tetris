use macroquad::input::KeyCode;

use crate::{
    block_controller::BlockController,
    constants::{
        colors::{BLACK, WHITE},
        font_constants::{
            GAME_OVER_BOTTOM_FONT_SIZE, GAME_OVER_BOTTOM_TEXT, GAME_OVER_FONT_SIZE,
            GAME_OVER_OUTLINE_WIDTH, GAME_OVER_TEXT,
        },
        map_constants::{MAP_HEIGHT, MAP_WIDTH},
    },
    graphic_controller::GraphicController,
    map::Map,
};

pub struct GameController {
    score: i32,
    is_game_over: bool,
    block_controller: BlockController,
    map: Map,
}

impl GameController {
    pub fn new() -> GameController {
        let block_controller: BlockController = BlockController::new();
        let map: Map = Map::new(MAP_WIDTH, MAP_HEIGHT);

        GameController {
            score: 0,
            is_game_over: false,
            block_controller,
            map,
        }
    }

    pub fn reset_game(&mut self) {
        self.score = 0;
        self.is_game_over = false;
        self.map.clear();
        self.block_controller.clear();
    }

    pub async fn tick(&mut self) {
        self.draw_game().await;

        if self.is_game_over {
            return ();
        }

        self.map.tick();

        let is_game_over = self
            .block_controller
            .tick_and_check_game_over(&mut self.map);
        if is_game_over {
            self.handle_game_over();
        }
    }

    fn handle_game_over(&mut self) {
        self.is_game_over = true;
    }

    fn display_game_over(&self) {
        let text_center = GraphicController::get_text_center(GAME_OVER_TEXT, GAME_OVER_FONT_SIZE);
        let bottom_text_center =
            GraphicController::get_text_center(GAME_OVER_BOTTOM_TEXT, GAME_OVER_BOTTOM_FONT_SIZE);
        let map_center = GraphicController::map_to_window_dimensions(MAP_WIDTH / 2, MAP_HEIGHT / 2);

        GraphicController::draw_text_with_outline(
            GAME_OVER_TEXT,
            map_center.0 - text_center.0,
            map_center.1 + text_center.1,
            GAME_OVER_FONT_SIZE,
            BLACK,
            WHITE,
            GAME_OVER_OUTLINE_WIDTH,
        );
        GraphicController::draw_text_with_outline(
            GAME_OVER_BOTTOM_TEXT,
            map_center.0 - bottom_text_center.0,
            map_center.1 + text_center.1 + 2.0 * bottom_text_center.1,
            GAME_OVER_BOTTOM_FONT_SIZE,
            BLACK,
            WHITE,
            GAME_OVER_OUTLINE_WIDTH,
        );
    }

    async fn draw_game(&mut self) {
        GraphicController::draw_background();
        GraphicController::draw_block(self.block_controller.get_block_to_draw());
        GraphicController::draw_fields(self.map.get_fields_to_draw());

        if self.is_game_over {
            self.display_game_over();
        }

        GraphicController::flush().await;
    }

    pub fn do_move(&mut self, key: KeyCode) {
        if self.is_game_over {
            return ();
        }
        let is_game_over = match key {
            KeyCode::D => self.block_controller.handle_move_right(&mut self.map),
            KeyCode::A => self.block_controller.handle_move_left(&mut self.map),
            KeyCode::S => self.block_controller.handle_move_down(&mut self.map),
            KeyCode::E => self.block_controller.handle_rotate_clockwise(&mut self.map),
            KeyCode::Q => self
                .block_controller
                .handle_rotate_counter_clockwise(&mut self.map),
            _ => false,
        };
        if is_game_over {
            self.handle_game_over();
        }
    }
}
